use crate::helpers::{read_citizen_cert, read_nation_cert};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Column, ConstraintSystem, Error, Expression, Instance, TableColumn},
    poly::Rotation,
};
use zkevm_gadgets::util::*;

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
    pub last_type: Column<Advice>,
    pub parsed_bytes: Column<Advice>,
    pub parsed_objects: Column<Advice>,
    pub primitive_types: TableColumn,
    pub composite_types: TableColumn,
    pub is_primitive: Column<Advice>,
    // 0..256
    pub byte: TableColumn,
    pub is_below_128: Column<Advice>,
    pub public_objects: Column<Instance>,
    pub private_objects: Column<Instance>,
    pub should_disclose: Column<Advice>,
    pub disclosure: Column<Instance>,
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
            last_type: meta.advice_column(),
            parsed_bytes: meta.advice_column(),
            parsed_objects: meta.advice_column(),
            primitive_types: meta.lookup_table_column(),
            composite_types: meta.lookup_table_column(),
            is_primitive: meta.advice_column(),
            byte: meta.lookup_table_column(),
            is_below_128: meta.advice_column(),
            public_objects: meta.instance_column(),
            private_objects: meta.instance_column(),
            should_disclose: meta.advice_column(),
            disclosure: meta.instance_column(),
        };

        meta.create_gate("DerChip", |gate| {
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            let is_primitive = gate.query_advice(config.is_primitive, Rotation::cur());
            let is_below_128 = gate.query_advice(config.is_below_128, Rotation::cur());
            let should_disclose = gate.query_advice(config.should_disclose, Rotation::cur());
            vec![
                // is_type must be either 0 or 1.
                is_type.clone() * (is_type.clone() - Expression::Constant(Fr::one())),
                // is_length must be either 0 or 1.
                is_length.clone() * (is_length.clone() - Expression::Constant(Fr::one())),
                // is_payload must be either 0 or 1.
                is_payload.clone() * (is_payload.clone() - Expression::Constant(Fr::one())),
                // is_primitive must be either 0 or 1.
                is_primitive.clone() * (is_primitive.clone() - Expression::Constant(Fr::one())),
                // is_below_128 must be either 0 or 1.
                is_below_128.clone() * (is_below_128.clone() - Expression::Constant(Fr::one())),
                // should_disclose must be either 0 or 1.
                should_disclose.clone() * (should_disclose.clone() - Expression::Constant(Fr::one())),
            ]
        });

        meta.lookup("der_byte <= 0b11111111", |region| {
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            vec![(der_byte, config.byte)]
        });

        meta.lookup("last_type <= 0b11111111", |region| {
            let last_type = region.query_advice(config.last_type, Rotation::cur());
            vec![(last_type, config.byte)]
        });

        meta.lookup("last_type <= 0b11111 if is_primitive is 1", |region| {
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            let last_type = region.query_advice(config.last_type, Rotation::cur());
            let shifted = last_type * Expression::Constant(Fr::from(1 << 3));
            vec![(is_primitive * shifted, config.byte)]
        });

        meta.lookup("0b100000 <= last_type <= 0b111111 if is_primitive is 0", |region| {
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            let is_composite = Expression::Constant(Fr::one()) - is_primitive;
            let last_type = region.query_advice(config.last_type, Rotation::cur());
            let shifted = last_type * Expression::Constant(Fr::from(1 << 3)) - Expression::Constant(Fr::from(1 << 8));
            vec![(is_composite * shifted, config.byte)]
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
            let shifted = der_byte - Expression::Constant(Fr::from(1 << 8));
            vec![(is_above_128 * shifted, config.byte)]
        });

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

        config
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<Fr>) -> Result<(), Error> {
        layouter.assign_region(
            || "DerChip",
            |mut region| {
                // Assign der_byte.
                for (i, der_byte) in self.der_bytes.iter().copied().enumerate() {
                    region.assign_advice(|| "aa", config.der_byte, i, || Value::known(Fr::from(der_byte as u64)))?;
                }

                Ok(())
            },
        )?;

        layouter.assign_table(
            || "DerChip",
            |mut table| {
                for row_index in 0..(1 << Self::K) {
                    // Assign byte table.
                    let row_value = row_index & 0b11111111;
                    table.assign_cell(
                        || "DerChip",
                        config.byte,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    // Assign primitive types.
                    let row_value = row_index & 0b11111;
                    table.assign_cell(
                        || "DerChip",
                        config.primitive_types,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    // Assign composite types.
                    let row_value = 0b100000 + (row_index & 0b11111);
                    table.assign_cell(
                        || "DerChip",
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
