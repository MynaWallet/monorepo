use crate::helpers::{read_citizen_cert, read_nation_cert};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Assigned, Column, ConstraintSystem, Error, Expression, Fixed, Instance, TableColumn},
    poly::Rotation,
};
use zkevm_gadgets::util::*;

// DER type tag.
const OCTET_STRING: u8 = 0x04;

#[derive(Debug, Clone)]
pub struct Config {
    // The DER we're parsing. each cell corresponds to a byte.
    pub der_byte: Column<Advice>,
    // 1 if the byte we're parsing is in a position where the type tag should be.
    pub is_type: Column<Advice>,
    // 1 if the byte we're parsing is in a position where the length of payload should be.
    pub is_length: Column<Advice>,
    // 1 if the byte we're parsing is in a position where the payload should be.
    pub is_payload: Column<Advice>,
    // The length (bytes) of the header in the object we're parsing.
    pub header_size: Column<Advice>,
    // The length (bytes) of the payload in the object we're parsing.
    pub payload_size: Column<Advice>,
    // The number of parsed object in the entire DER.
    pub type_tag: Column<Advice>,
    pub parsed_bytes: Column<Advice>,
    pub parsed_objects: Column<Advice>,
    pub is_primitive: Column<Advice>,
    pub primitive_types: TableColumn,
    pub composite_types: TableColumn,
    // 0..256
    pub byte: TableColumn,
    pub is_below_128: Column<Advice>,
    pub public_objects: Column<Instance>,
    pub private_objects: Column<Instance>,
    pub should_disclose: Column<Advice>,
    pub disclosure: Column<Instance>,
    pub constants: Column<Fixed>,
}

#[derive(Debug, Clone)]
pub struct Circuit {
    der_bytes: Vec<u8>,
}

impl Circuit {
    const K: usize = 14;
}

impl plonk::Circuit<Fr> for Circuit {
    type Config = Config;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        unreachable!();
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let config = Self::Config {
            der_byte: meta.advice_column(),
            is_type: meta.advice_column(),
            is_length: meta.advice_column(),
            is_payload: meta.advice_column(),
            header_size: meta.advice_column(),
            payload_size: meta.advice_column(),
            type_tag: meta.advice_column(),
            parsed_bytes: meta.advice_column(),
            parsed_objects: meta.advice_column(),
            is_primitive: meta.advice_column(),
            primitive_types: meta.lookup_table_column(),
            composite_types: meta.lookup_table_column(),
            byte: meta.lookup_table_column(),
            is_below_128: meta.advice_column(),
            public_objects: meta.instance_column(),
            private_objects: meta.instance_column(),
            should_disclose: meta.advice_column(),
            disclosure: meta.instance_column(),
            constants: meta.fixed_column(),
        };
        meta.enable_constant(config.constants);
        meta.enable_equality(config.constants);

        meta.create_gate("Selector columns must be either 0 or 1", |gate| {
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            let is_primitive = gate.query_advice(config.is_primitive, Rotation::cur());
            let is_below_128 = gate.query_advice(config.is_below_128, Rotation::cur());
            let should_disclose = gate.query_advice(config.should_disclose, Rotation::cur());
            vec![
                is_type.clone() * (is_type.clone() - Expression::Constant(Fr::one())),
                is_length.clone() * (is_length.clone() - Expression::Constant(Fr::one())),
                is_payload.clone() * (is_payload.clone() - Expression::Constant(Fr::one())),
                is_primitive.clone() * (is_primitive.clone() - Expression::Constant(Fr::one())),
                is_below_128.clone() * (is_below_128.clone() - Expression::Constant(Fr::one())),
                should_disclose.clone() * (should_disclose.clone() - Expression::Constant(Fr::one())),
            ]
        });

        meta.create_gate("Either one of is_type, is_length, is_payload must be turned on", |gate| {
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            vec![
                is_type.clone() * is_length.clone(),
                is_type.clone() * is_payload.clone(),
                is_length.clone() * is_type.clone(),
                is_length.clone() * is_payload.clone(),
                is_payload.clone() * is_type.clone(),
                is_payload.clone() * is_length.clone(),
            ]
        });

        meta.create_gate("We can't turn the parser down unless der_byte tells so", |gate| {
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            vec![
                is_type.clone() * is_length.clone(),
                is_type.clone() * is_payload.clone(),
                is_length.clone() * is_type.clone(),
                is_length.clone() * is_payload.clone(),
                is_payload.clone() * is_type.clone(),
                is_payload.clone() * is_length.clone(),
            ]
        });

        meta.lookup("der_byte <= 0b11111111", |region| {
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            vec![(der_byte, config.byte)]
        });

        meta.lookup("type_tag <= 0b11111111", |region| {
            let type_tag = region.query_advice(config.type_tag, Rotation::cur());
            vec![(type_tag, config.byte)]
        });

        meta.lookup("type_tag must be in primitive_types if is_primitive is 1", |region| {
            let type_tag = region.query_advice(config.type_tag, Rotation::cur());
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            vec![(is_primitive * type_tag, config.primitive_types)]
        }); // 0 is in primitive_types so this lookup suceeds if is_primitive = 0

        meta.lookup("type_tag must be in composite_types if is_primitive is 0", |region| {
            let type_tag = region.query_advice(config.type_tag, Rotation::cur());
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            let is_composite = Expression::Constant(Fr::one()) - is_primitive;
            vec![(
                select::expr(is_composite, type_tag, Expression::Constant(Fr::from(OCTET_STRING as u64))),
                config.composite_types,
            )]
        });

        meta.lookup("der_byte <= 0b1111111 if is_below_128 is 1", |region| {
            let is_below_128 = region.query_advice(config.is_below_128, Rotation::cur());
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            let shifted = der_byte * Expression::Constant(Fr::from(1 << 1));
            vec![(is_below_128 * shifted, config.byte)]
        });

        meta.lookup("der_byte > 0b1111111 if is_below_128 is 0", |region| {
            let is_below_128 = region.query_advice(config.is_below_128, Rotation::cur());
            let is_above_128 = Expression::Constant(Fr::one()) - is_below_128;
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            let shifted = der_byte - Expression::Constant(Fr::from(1 << 7));
            vec![(is_above_128 * shifted, config.byte)]
        });

        // assumption here is that parsed_objects will never be 0.
        // If parsed_objects became 0 both lookup would succeed,
        // and there would be no constraint on whether should_disclose should be 0 or 1.
        // Exception is that the first row where parsed_objects is constrained to be 0.
        // The case of the first row is fine because is_payload is also constrained to be 0.
        // If is_payload = 0 no byte will be disclosed anyway.
        meta.lookup_any("parsed_objects must be in public_objects if should_disclose is 1", |region| {
            let parsed_objects = region.query_advice(config.parsed_objects, Rotation::cur());
            let public_objects = region.query_instance(config.public_objects, Rotation::cur());
            let should_disclose = region.query_advice(config.should_disclose, Rotation::cur());
            vec![(should_disclose * parsed_objects, public_objects)]
        });

        meta.lookup_any("parsed_objects must be in private_objects if should_disclose is 0", |region| {
            let parsed_objects = region.query_advice(config.parsed_objects, Rotation::cur());
            let private_objects = region.query_instance(config.private_objects, Rotation::cur());
            let should_disclose = region.query_advice(config.should_disclose, Rotation::cur());
            let should_hide = Expression::Constant(Fr::one()) - should_disclose;
            vec![(should_hide * parsed_objects, private_objects)]
        });

        meta.create_gate("Constrain order of operations", |region| {
            let is_payload_prev = region.query_advice(config.is_payload, Rotation::prev());
            let is_payload_cur = region.query_advice(config.is_payload, Rotation::cur());
            let parsed_bytes_cur = region.query_advice(config.parsed_bytes, Rotation::cur());
            let payload_size_cur = region.query_advice(config.payload_size, Rotation::cur());
            let is_length_cur = region.query_advice(config.is_length, Rotation::cur());
            let is_length_prev = region.query_advice(config.is_length, Rotation::prev());
            let is_type_cur = region.query_advice(config.is_type, Rotation::cur());
            let is_type_prev = region.query_advice(config.is_type, Rotation::prev());
            let is_primitive_cur = region.query_advice(config.is_primitive, Rotation::cur());
            let header_size_cur = region.query_advice(config.header_size, Rotation::cur());
            vec![
                // If is_type is turned on, the previous is_type must be turned off.
                // Because a type tag consumes exactly 1 bit.
                is_type_cur.clone() * is_type_prev.clone(),
                // If is_length is turned on, the previous is_payload must be turned off.
                // Because there's no such case that a length byte is followed by a payload byte.
                is_length_cur.clone() * is_payload_prev.clone(),
                // If is_payload is turned on, the previous is_type must be turned off.
                // Because there's no such case that a payload byte is followed by a type byte.
                is_payload_cur.clone() * is_type_prev.clone(),
                // If is_length_prev = 1 && is_length_cur = 0 && is_primitive_cur = 1, then is_payload_cur must be 1
                // Because in a primitive object payload bytes follows length bytes
                is_length_prev.clone()
                    * not::expr(is_length_cur.clone())
                    * is_primitive_cur.clone()
                    * not::expr(is_payload_cur.clone()),
                // If is_length_prev = 1 && is_length_cur = 0 && is_primitive_cur = 0, then is_type_cur must be 1
                // Because in an object that's not primitive a type byte follows length bytes
                is_length_prev.clone()
                    * not::expr(is_length_cur.clone())
                    * not::expr(is_primitive_cur.clone())
                    * not::expr(is_type_cur.clone()),
                // If is_length_prev = 1 && is_length_cur = 0, then parsed_bytes must equal header_size
                // This is to prevent an attacker from stop parsing length bytes early.
                is_length_prev.clone()
                    * not::expr(is_length_cur.clone())
                    * (parsed_bytes_cur.clone() - header_size_cur.clone()),
                // If is_payload_prev = 1 && is_payload_cur = 0,
                // then parsed_bytes must equal header_size + payload_size.
                // This is to prevent an attacker from stop parsing payload bytes early.
                is_payload_prev.clone()
                    * not::expr(is_payload_cur.clone())
                    * (parsed_bytes_cur.clone() - header_size_cur.clone() - payload_size_cur.clone()),
            ]
        });

        meta.create_gate("Constrain value of registers", |region| {
            let disclosure_cur = region.query_instance(config.disclosure, Rotation::next());
            let is_payload_cur = region.query_advice(config.is_payload, Rotation::cur());
            let parsed_bytes_next = region.query_advice(config.parsed_bytes, Rotation::next());
            let parsed_bytes_cur = region.query_advice(config.parsed_bytes, Rotation::cur());
            let parsed_objects_next = region.query_advice(config.parsed_objects, Rotation::next());
            let parsed_objects_cur = region.query_advice(config.parsed_objects, Rotation::cur());
            let type_tag_next = region.query_advice(config.type_tag, Rotation::next());
            let type_tag_cur = region.query_advice(config.type_tag, Rotation::cur());
            let header_size_next = region.query_advice(config.header_size, Rotation::next());
            let header_size_cur = region.query_advice(config.header_size, Rotation::cur());
            let payload_size_next = region.query_advice(config.payload_size, Rotation::next());
            let payload_size_cur = region.query_advice(config.payload_size, Rotation::cur());
            let is_length_cur = region.query_advice(config.is_length, Rotation::cur());
            let is_type_cur = region.query_advice(config.is_type, Rotation::cur());
            let is_type_prev = region.query_advice(config.is_type, Rotation::prev());
            let is_below_128_cur = region.query_advice(config.is_below_128, Rotation::cur());
            let der_byte_cur = region.query_advice(config.der_byte, Rotation::cur());
            let should_disclose_cur = region.query_advice(config.should_disclose, Rotation::cur());
            vec![
                // if is_type = 1, reset the register. otherwise keep the value
                type_tag_next - select::expr(is_type_cur.clone(), der_byte_cur.clone(), type_tag_cur.clone()),
                parsed_bytes_next
                    - select::expr(
                        is_type_cur.clone(),
                        // If is_type = 1, reset the register with 1.
                        Expression::Constant(Fr::one()),
                        // otherwise, increment the register
                        parsed_bytes_cur.clone() + Expression::Constant(Fr::one()),
                    ),
                parsed_objects_next
                    - select::expr(
                        is_type_cur.clone(),
                        // If is_type = 1, increment the register.
                        parsed_objects_cur.clone() + Expression::Constant(Fr::one()),
                        // otherwise, keep the value
                        parsed_objects_cur.clone(),
                    ),
                header_size_next
                    - select::expr(
                        is_type_cur.clone(),
                        // is_type = 1, reset the register with 2.
                        Expression::Constant(Fr::from(2)),
                        select::expr(
                            // If it's the first row of length bytes and the last bit of der_byte is turned on,
                            and::expr([
                                is_length_cur.clone(),
                                is_type_prev.clone(),
                                not::expr(is_below_128_cur.clone()),
                            ]),
                            // The first 7 bits of der_byte indicates length of length bytes.
                            header_size_cur.clone() + der_byte_cur.clone() - Expression::Constant(Fr::from(1 << 7)),
                            // otherwise keep the value
                            header_size_cur.clone(),
                        ),
                    ),
                payload_size_next
                    - select::expr(
                        is_type_cur.clone(),
                        // If is_type = 1, reset the register with 0
                        Expression::Constant(Fr::zero()),
                        select::expr(
                            is_length_cur.clone(),
                            select::expr(
                                // If it's the first row of length bytes,
                                is_type_prev.clone(),
                                // If der_byte's 8th bit is turned off, reset the register with der_byte.
                                // otherwise, reset the register with 0
                                is_below_128_cur.clone() * der_byte_cur.clone(),
                                // If it's not the first row of length bytes, push those bytes into payload_size
                                // accumulator. big endian.
                                payload_size_cur.clone() * Expression::Constant(Fr::from(1 << 8))
                                    + der_byte_cur.clone(),
                            ),
                            // otherwise keep the value
                            payload_size_cur.clone(),
                        ),
                    ),
                disclosure_cur.clone()
                    - select::expr(
                        // If we should disclose the byte
                        and::expr([should_disclose_cur.clone(), is_payload_cur.clone()]),
                        // We disclose the byte
                        der_byte_cur.clone(),
                        // Otherwise we write a value that's over MAX_BYTE to indicate that those bytes are redacted
                        Expression::Constant(Fr::from(1 << 8)),
                    ),
            ]
        });

        config
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<Fr>) -> Result<(), Error> {
        layouter.assign_region(
            || "DerChip",
            |mut region| {
                for (i, der_byte) in self.der_bytes.iter().copied().enumerate() {
                    region.assign_advice(
                        || "Assign each byte of the DER as a cell",
                        config.der_byte,
                        i,
                        || Value::known(Fr::from(der_byte as u64)),
                    )?;
                }
                region.assign_advice_from_constant(
                    || "The first op must be PARSE_TYPE",
                    config.is_type,
                    0,
                    Assigned::Trivial(Fr::one()),
                )?;
                region.assign_advice_from_constant(
                    || "The first parsed_objects must be 0",
                    config.parsed_objects,
                    0,
                    Assigned::Trivial(Fr::zero()),
                )?;
                region.assign_advice_from_constant(
                    || "The first parsed_bytes must be 0",
                    config.parsed_bytes,
                    0,
                    Assigned::Trivial(Fr::zero()),
                )?;
                region.assign_advice_from_constant(
                    || "The first header_size must be 2",
                    config.header_size,
                    0,
                    Assigned::Trivial(Fr::from(2)),
                )?;
                region.assign_advice_from_constant(
                    || "The first payload_size must be 0",
                    config.payload_size,
                    0,
                    Assigned::Trivial(Fr::from(0)),
                )?;
                region.assign_advice_from_constant(
                    || "The first type_tag must be OCTET_STRING",
                    config.payload_size,
                    0,
                    Assigned::Trivial(Fr::from(OCTET_STRING as u64)),
                )?;

                Ok(())
            },
        )?;

        layouter.assign_table(
            || "DerChip",
            |mut table| {
                for row_index in 0..(1 << Self::K) {
                    let row_value = row_index & 0b11111111;
                    table.assign_cell(
                        || "List of all possible bytes",
                        config.byte,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    let row_value =
                        if row_index == OCTET_STRING as usize || (1 << 5) <= row_index { 0 } else { row_index };
                    table.assign_cell(
                        || "List of type tags for primitive types",
                        config.primitive_types,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    let row_value = if row_index == OCTET_STRING as usize || (1 << 5) > row_index {
                        OCTET_STRING as usize
                    } else {
                        row_index
                    };
                    table.assign_cell(
                        || "List of type tags for composite types",
                        config.composite_types,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}
