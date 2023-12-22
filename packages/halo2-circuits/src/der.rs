use std::collections::HashSet;

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Column, ConstraintSystem, Error, Expression, Fixed, Instance, Selector, TableColumn},
    poly::Rotation,
};
use zkevm_gadgets::util::*;

// DER type tag.
const OCTET_STRING: u8 = 0x04;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Action {
    Parse(u8),
    DoNothing,
}

impl Action {
    fn der_byte(self) -> u64 {
        if let Action::Parse(byte) = self { byte as u64 } else { 1u64 << 8 }
    }

    fn is_below_128(self) -> bool {
        match self {
            Self::Parse(der_byte) => der_byte < 128,
            Self::DoNothing => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum ByteKind {
    Type,
    Length,
    Payload,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct State {
    header_size: u32,
    payload_size: u32,
    parsed_bytes: u32,
    parsed_objects: u32,
    type_tag: u8,
    byte_kind: ByteKind,
}

impl State {
    fn new() -> State {
        State {
            header_size: 0,
            payload_size: 0,
            parsed_bytes: 0,
            parsed_objects: 0,
            type_tag: 0,
            byte_kind: ByteKind::Type,
        }
    }

    fn is_type(self) -> bool {
        self.byte_kind == ByteKind::Type
    }

    fn is_length(self) -> bool {
        self.byte_kind == ByteKind::Length
    }

    fn is_payload(self) -> bool {
        self.byte_kind == ByteKind::Payload
    }

    fn is_primitive(self) -> bool {
        self.type_tag < 1 << 5 && self.type_tag != OCTET_STRING
    }

    fn update(self, action: Action) -> State {
        let mut new_state = self.clone();
        new_state.parsed_bytes += 1;

        if self.byte_kind == ByteKind::Type {
            new_state.byte_kind = ByteKind::Length;
            new_state.type_tag = match action {
                Action::Parse(byte) => byte,
                Action::DoNothing => 0,
            };
            new_state.parsed_bytes = 1;
            new_state.header_size = 2;
            new_state.payload_size = 0;
            new_state.parsed_objects += 1;
        } else if self.byte_kind == ByteKind::Length && self.parsed_bytes == self.header_size - 1 {
            new_state.byte_kind = ByteKind::Payload;
        } else if self.byte_kind == ByteKind::Payload && self.parsed_bytes == self.header_size + self.payload_size - 1 {
            new_state.byte_kind = ByteKind::Type;
        };

        if let Action::Parse(byte) = action {
            if self.byte_kind == ByteKind::Length {
                if self.parsed_bytes == 1 {
                    if action.is_below_128() {
                        new_state.payload_size = byte as u32;
                    } else {
                        new_state.header_size = (byte - (1 << 7) + 2) as u32;
                    }
                } else {
                    new_state.payload_size *= 1u32 << 8;
                    new_state.payload_size += byte as u32;
                }
            }
        }

        new_state
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub is_enabled: Selector,
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
    public_objects: HashSet<u32>,
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
            is_enabled: meta.selector(),
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
            let is_enabled = gate.query_selector(config.is_enabled);
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            let is_primitive = gate.query_advice(config.is_primitive, Rotation::cur());
            let is_below_128 = gate.query_advice(config.is_below_128, Rotation::cur());
            let should_disclose = gate.query_advice(config.should_disclose, Rotation::cur());
            vec![
                is_enabled.clone() * (is_enabled.clone() - Expression::Constant(Fr::one())),
                is_enabled.clone() * is_type.clone() * (is_type.clone() - Expression::Constant(Fr::one())),
                is_enabled.clone() * is_length.clone() * (is_length.clone() - Expression::Constant(Fr::one())),
                is_enabled.clone() * is_payload.clone() * (is_payload.clone() - Expression::Constant(Fr::one())),
                is_enabled.clone() * is_primitive.clone() * (is_primitive.clone() - Expression::Constant(Fr::one())),
                is_enabled.clone() * is_below_128.clone() * (is_below_128.clone() - Expression::Constant(Fr::one())),
                is_enabled.clone()
                    * should_disclose.clone()
                    * (should_disclose.clone() - Expression::Constant(Fr::one())),
            ]
        });

        meta.create_gate("Either one of is_type, is_length, is_payload must be turned on", |gate| {
            let is_enabled = gate.query_selector(config.is_enabled);
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            vec![
                is_enabled.clone() * is_type.clone() * is_length.clone(),
                is_enabled.clone() * is_type.clone() * is_payload.clone(),
                is_enabled.clone() * is_length.clone() * is_type.clone(),
                is_enabled.clone() * is_length.clone() * is_payload.clone(),
                is_enabled.clone() * is_payload.clone() * is_type.clone(),
                is_enabled.clone() * is_payload.clone() * is_length.clone(),
                is_enabled.clone() * is_type.clone() * is_length.clone() * is_payload.clone(),
            ]
        });

        meta.lookup("der_byte <= 0b11111111", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            vec![(is_enabled * der_byte, config.byte)]
        });

        meta.lookup("type_tag <= 0b11111111", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let type_tag = region.query_advice(config.type_tag, Rotation::cur());
            vec![(is_enabled * type_tag, config.byte)]
        });

        meta.lookup("type_tag must be in primitive_types if is_primitive is 1", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let type_tag = region.query_advice(config.type_tag, Rotation::cur());
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            vec![(is_enabled * is_primitive * type_tag, config.primitive_types)]
        }); // 0 is in primitive_types so this lookup succeeds if is_primitive = 0

        meta.lookup("type_tag must be in composite_types if is_primitive is 0", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let type_tag = region.query_advice(config.type_tag, Rotation::cur());
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            let is_composite = Expression::Constant(Fr::one()) - is_primitive;
            vec![(
                select::expr(
                    and::expr([is_enabled, is_composite]),
                    type_tag,
                    Expression::Constant(Fr::from(OCTET_STRING as u64)),
                ),
                config.composite_types,
            )]
        });

        meta.lookup("der_byte <= 0b1111111 if is_below_128 is 1", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let is_below_128 = region.query_advice(config.is_below_128, Rotation::cur());
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            let shifted = der_byte * Expression::Constant(Fr::from(1 << 1));
            vec![(is_enabled * is_below_128 * shifted, config.byte)]
        });

        meta.lookup("der_byte > 0b1111111 if is_below_128 is 0", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let is_below_128 = region.query_advice(config.is_below_128, Rotation::cur());
            let is_above_128 = Expression::Constant(Fr::one()) - is_below_128;
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            let shifted = der_byte - Expression::Constant(Fr::from(1 << 7));
            vec![(is_enabled * is_above_128 * shifted, config.byte)]
        });

        // assumption here is that parsed_objects will never be 0.
        // If parsed_objects became 0 both lookup would succeed,
        // and there would be no constraint on whether should_disclose should be 0 or 1.
        // Exception is that the first row where parsed_objects is constrained to be 0.
        // The case of the first row is fine because is_payload is also constrained to be 0.
        // If is_payload = 0 no byte will be disclosed anyway.
        meta.lookup_any("parsed_objects must be in public_objects if should_disclose is 1", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_objects = region.query_advice(config.parsed_objects, Rotation::cur());
            let public_objects = region.query_instance(config.public_objects, Rotation::cur());
            let should_disclose = region.query_advice(config.should_disclose, Rotation::cur());
            vec![(is_enabled * should_disclose * parsed_objects, public_objects)]
        });

        meta.lookup_any("parsed_objects must be in private_objects if should_disclose is 0", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_objects = region.query_advice(config.parsed_objects, Rotation::cur());
            let private_objects = region.query_instance(config.private_objects, Rotation::cur());
            let should_disclose = region.query_advice(config.should_disclose, Rotation::cur());
            let should_hide = Expression::Constant(Fr::one()) - should_disclose;
            vec![(is_enabled * should_hide * parsed_objects, private_objects)]
        });

        meta.create_gate("Constrain order of operations", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
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
                is_enabled.clone() * is_type_cur.clone() * is_type_prev.clone(),
                // If is_length is turned on, the previous is_payload must be turned off.
                // Because there's no such case that a length byte is followed by a payload byte.
                is_enabled.clone() * is_length_cur.clone() * is_payload_prev.clone(),
                // If is_payload is turned on, the previous is_type must be turned off.
                // Because there's no such case that a payload byte is followed by a type byte.
                is_enabled.clone() * is_payload_cur.clone() * is_type_prev.clone(),
                // If is_length_prev = 1 && is_length_cur = 0 && is_primitive_cur = 1, then is_payload_cur must be 1
                // Because in a primitive object payload bytes follows length bytes
                is_enabled.clone()
                    * is_length_prev.clone()
                    * not::expr(is_length_cur.clone())
                    * is_primitive_cur.clone()
                    * not::expr(is_payload_cur.clone()),
                // If is_length_prev = 1 && is_length_cur = 0 && is_primitive_cur = 0, then is_type_cur must be 1
                // Because in an object that's not primitive a type byte follows length bytes
                is_enabled.clone()
                    * is_length_prev.clone()
                    * not::expr(is_length_cur.clone())
                    * not::expr(is_primitive_cur.clone())
                    * not::expr(is_type_cur.clone()),
                // If is_length_prev = 1 && is_length_cur = 0, then parsed_bytes must equal header_size
                // This is to prevent an attacker from stop parsing length bytes early.
                is_enabled.clone()
                    * is_length_prev.clone()
                    * not::expr(is_length_cur.clone())
                    * (parsed_bytes_cur.clone() - header_size_cur.clone()),
                // If is_payload_prev = 1 && is_payload_cur = 0,
                // then parsed_bytes must equal header_size + payload_size.
                // This is to prevent an attacker from stop parsing payload bytes early.
                is_enabled.clone()
                    * is_payload_prev.clone()
                    * not::expr(is_payload_cur.clone())
                    * (parsed_bytes_cur.clone() - header_size_cur.clone() - payload_size_cur.clone()),
            ]
        });

        meta.create_gate("Constrain value of registers", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let disclosure_next = region.query_instance(config.disclosure, Rotation::next());
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
                is_enabled.clone()
                    * (type_tag_next - select::expr(is_type_cur.clone(), der_byte_cur.clone(), type_tag_cur.clone())),
                is_enabled.clone()
                    * (parsed_bytes_next
                        - select::expr(
                            is_type_cur.clone(),
                            // If is_type = 1, reset the register with 1.
                            Expression::Constant(Fr::one()),
                            // otherwise, increment the register
                            parsed_bytes_cur.clone() + Expression::Constant(Fr::one()),
                        )),
                is_enabled.clone()
                    * (parsed_objects_next
                        - select::expr(
                            is_type_cur.clone(),
                            // If is_type = 1, increment the register.
                            parsed_objects_cur.clone() + Expression::Constant(Fr::one()),
                            // otherwise, keep the value
                            parsed_objects_cur.clone(),
                        )),
                is_enabled.clone()
                    * (header_size_next
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
                        )),
                is_enabled.clone()
                    * (payload_size_next
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
                        )),
                is_enabled.clone()
                    * (disclosure_next.clone()
                        - select::expr(
                            // If we should disclose the byte
                            and::expr([should_disclose_cur.clone(), is_payload_cur.clone()]),
                            // We disclose the byte
                            der_byte_cur.clone(),
                            // Otherwise we write a value that's over MAX_BYTE to indicate that those bytes are
                            // redacted
                            Expression::Constant(Fr::from(1 << 8)),
                        )),
            ]
        });

        config
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<Fr>) -> Result<(), Error> {
        layouter.assign_region(
            || "DerChip",
            |mut region| {
                // Enable is_enable
                config.is_enabled.enable(&mut region, 0)?;

                // Assign initial state
                let mut state = State::new();
                region.assign_advice_from_constant(
                    || "Assign is_type",
                    config.is_type,
                    0,
                    Fr::from(state.is_type()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign is_length",
                    config.is_length,
                    0,
                    Fr::from(state.is_length()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign is_payload",
                    config.is_payload,
                    0,
                    Fr::from(state.is_payload()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign header_size",
                    config.header_size,
                    0,
                    Fr::from(state.header_size as u64),
                )?;
                region.assign_advice_from_constant(
                    || "Assign payload_size",
                    config.payload_size,
                    0,
                    Fr::from(state.payload_size as u64),
                )?;
                region.assign_advice_from_constant(
                    || "Assign parsed_bytes",
                    config.parsed_bytes,
                    0,
                    Fr::from(state.parsed_bytes as u64),
                )?;
                region.assign_advice_from_constant(
                    || "Assign parsed_objects",
                    config.parsed_objects,
                    0,
                    Fr::from(state.parsed_objects as u64),
                )?;
                region.assign_advice_from_constant(
                    || "Assign type_tag",
                    config.type_tag,
                    0,
                    Fr::from(state.type_tag as u64),
                )?;
                region.assign_advice_from_constant(
                    || "Assign is_primitive",
                    config.is_primitive,
                    0,
                    Fr::from(state.is_primitive()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign should_disclose",
                    config.should_disclose,
                    0,
                    Fr::from(self.should_disclose(state.parsed_objects)),
                )?;

                for row_index in 1..(1 << Self::K) - Self::K {
                    // Assign action
                    let action = if let Some(byte) = self.der_bytes.get(row_index) {
                        Action::Parse(*byte)
                    } else {
                        Action::DoNothing
                    };
                    region.assign_advice(
                        || "Assign der_byte",
                        config.der_byte,
                        row_index - 1,
                        || Value::known(Fr::from(action.der_byte())),
                    )?;
                    region.assign_advice(
                        || "Assign is_below_128",
                        config.is_below_128,
                        row_index - 1,
                        || Value::known(Fr::from(action.is_below_128())),
                    )?;

                    // Enable is_enable
                    if row_index < (1 << Self::K) - Self::K {
                        config.is_enabled.enable(&mut region, row_index)?;
                    }

                    // Assign new state
                    state = state.update(action);
                    region.assign_advice(
                        || "Assign is_type",
                        config.is_type,
                        row_index,
                        || Value::known(Fr::from(state.is_type())),
                    )?;
                    region.assign_advice(
                        || "Assign is_length",
                        config.is_length,
                        row_index,
                        || Value::known(Fr::from(state.is_length())),
                    )?;
                    region.assign_advice(
                        || "Assign is_payload",
                        config.is_payload,
                        row_index,
                        || Value::known(Fr::from(state.is_payload())),
                    )?;
                    region.assign_advice(
                        || "Assign header_size",
                        config.header_size,
                        row_index,
                        || Value::known(Fr::from(state.header_size as u64)),
                    )?;
                    region.assign_advice(
                        || "Assign payload_size",
                        config.payload_size,
                        row_index,
                        || Value::known(Fr::from(state.payload_size as u64)),
                    )?;
                    region.assign_advice(
                        || "Assign parsed_bytes",
                        config.parsed_bytes,
                        row_index,
                        || Value::known(Fr::from(state.parsed_bytes as u64)),
                    )?;
                    region.assign_advice(
                        || "Assign parsed_objects",
                        config.parsed_objects,
                        row_index,
                        || Value::known(Fr::from(state.parsed_objects as u64)),
                    )?;
                    region.assign_advice(
                        || "Assign type_tag",
                        config.type_tag,
                        row_index,
                        || Value::known(Fr::from(state.type_tag as u64)),
                    )?;
                    region.assign_advice(
                        || "Assign is_primitive",
                        config.is_primitive,
                        row_index,
                        || Value::known(Fr::from(state.is_primitive())),
                    )?;
                    region.assign_advice(
                        || "Assign should_disclose",
                        config.should_disclose,
                        row_index,
                        || Value::known(Fr::from(self.should_disclose(state.parsed_objects))),
                    )?;
                }

                // Assign the last action.
                region.assign_advice_from_constant(
                    || "Assign der_byte",
                    config.der_byte,
                    (1 << Self::K) - Self::K,
                    Fr::from(Action::DoNothing.der_byte()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign is_below_128",
                    config.is_below_128,
                    (1 << Self::K) - Self::K,
                    Fr::from(Action::DoNothing.is_below_128()),
                )?;

                Ok(())
            },
        )?;

        layouter.assign_table(
            || "DerChip",
            |mut table| {
                for row_index in 0..(1 << Self::K) - Self::K {
                    let row_value = row_index & 0b11111111;
                    table.assign_cell(
                        || "List of all possible bytes",
                        config.byte,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    let row_value =
                        if row_index == OCTET_STRING as usize || row_index >= 1 << 5 { 0 } else { row_index };
                    table.assign_cell(
                        || "List of type tags for primitive types",
                        config.primitive_types,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    let row_value = if row_index == OCTET_STRING as usize || row_index < 1 << 5 {
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

impl Circuit {
    fn should_disclose(&self, parsed_objects: u32) -> bool {
        self.public_objects.contains(&parsed_objects)
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, State};
    use halo2_proofs::{dev::MockProver, halo2curves::bn256::Fr};
    use std::collections::HashSet;

    #[test]
    fn mock_der() {
        let mut circuit = super::Circuit {
            der_bytes: vec![
                48, 130, 5, 22, 160, 3, 2, 1, 2, 2, 4, 1, 140, 69, 62, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1,
                11, 5, 0, 48, 129, 130, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 74, 80, 49, 13, 48, 11, 6, 3, 85, 4, 10,
                12, 4, 74, 80, 75, 73, 49, 37, 48, 35, 6, 3, 85, 4, 11, 12, 28, 74, 80, 75, 73, 32, 102, 111, 114, 32,
                117, 115, 101, 114, 32, 97, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105, 111, 110, 49, 61, 48,
                59, 6, 3, 85, 4, 11, 12, 52, 74, 97, 112, 97, 110, 32, 65, 103, 101, 110, 99, 121, 32, 102, 111, 114,
                32, 76, 111, 99, 97, 108, 32, 65, 117, 116, 104, 111, 114, 105, 116, 121, 32, 73, 110, 102, 111, 114,
                109, 97, 116, 105, 111, 110, 32, 83, 121, 115, 116, 101, 109, 115, 48, 30, 23, 13, 50, 48, 48, 53, 49,
                57, 49, 55, 51, 56, 53, 53, 90, 23, 13, 50, 52, 49, 50, 48, 55, 49, 52, 53, 57, 53, 57, 90, 48, 47, 49,
                11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 74, 80, 49, 32, 48, 30, 6, 3, 85, 4, 3, 12, 23, 51, 48, 56, 48, 49,
                52, 69, 52, 53, 74, 69, 70, 73, 71, 49, 52, 49, 48, 52, 48, 48, 51, 65, 48, 130, 1, 34, 48, 13, 6, 9,
                42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15, 0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 178, 55,
                225, 13, 227, 171, 175, 41, 174, 202, 35, 194, 10, 85, 43, 211, 37, 20, 108, 7, 86, 252, 91, 9, 123,
                153, 30, 126, 197, 105, 85, 30, 187, 29, 185, 123, 113, 61, 124, 102, 98, 77, 120, 175, 249, 6, 140,
                18, 228, 65, 221, 209, 133, 33, 164, 47, 105, 216, 92, 154, 12, 238, 112, 124, 146, 109, 2, 234, 7, 88,
                141, 57, 23, 51, 4, 217, 29, 227, 1, 115, 46, 175, 186, 144, 203, 215, 160, 164, 22, 179, 58, 84, 74,
                21, 149, 48, 169, 26, 166, 184, 197, 46, 76, 96, 234, 81, 32, 118, 180, 202, 143, 157, 97, 83, 42, 254,
                154, 97, 36, 212, 187, 134, 218, 196, 154, 194, 105, 75, 163, 228, 41, 115, 190, 139, 1, 116, 80, 167,
                241, 164, 154, 100, 86, 78, 113, 52, 130, 215, 211, 45, 4, 251, 69, 245, 105, 217, 134, 214, 241, 204,
                196, 75, 123, 113, 178, 123, 93, 134, 65, 21, 12, 17, 111, 133, 170, 17, 93, 206, 220, 231, 165, 240,
                198, 95, 4, 142, 203, 211, 166, 84, 194, 255, 229, 20, 236, 163, 82, 96, 39, 209, 127, 18, 207, 4, 75,
                146, 202, 115, 165, 65, 162, 32, 161, 190, 163, 249, 45, 196, 82, 1, 82, 59, 178, 175, 18, 137, 76,
                201, 129, 177, 93, 177, 159, 70, 170, 40, 108, 29, 104, 58, 45, 67, 129, 49, 86, 158, 159, 250, 20,
                205, 55, 213, 245, 212, 97, 55, 2, 3, 1, 0, 1, 163, 130, 2, 252, 48, 130, 2, 248, 48, 14, 6, 3, 85, 29,
                15, 1, 1, 255, 4, 4, 3, 2, 7, 128, 48, 19, 6, 3, 85, 29, 37, 4, 12, 48, 10, 6, 8, 43, 6, 1, 5, 5, 7, 3,
                2, 48, 73, 6, 3, 85, 29, 32, 1, 1, 255, 4, 63, 48, 61, 48, 59, 6, 11, 42, 131, 8, 140, 155, 85, 8, 5,
                1, 3, 30, 48, 44, 48, 42, 6, 8, 43, 6, 1, 5, 5, 7, 2, 1, 22, 30, 104, 116, 116, 112, 58, 47, 47, 119,
                119, 119, 46, 106, 112, 107, 105, 46, 103, 111, 46, 106, 112, 47, 99, 112, 115, 46, 104, 116, 109, 108,
                48, 129, 183, 6, 3, 85, 29, 18, 4, 129, 175, 48, 129, 172, 164, 129, 169, 48, 129, 166, 49, 11, 48, 9,
                6, 3, 85, 4, 6, 19, 2, 74, 80, 49, 39, 48, 37, 6, 3, 85, 4, 10, 12, 30, 229, 133, 172, 231, 154, 132,
                229, 128, 139, 228, 186, 186, 232, 170, 141, 232, 168, 188, 227, 130, 181, 227, 131, 188, 227, 131,
                147, 227, 130, 185, 49, 57, 48, 55, 6, 3, 85, 4, 11, 12, 48, 229, 133, 172, 231, 154, 132, 229, 128,
                139, 228, 186, 186, 232, 170, 141, 232, 168, 188, 227, 130, 181, 227, 131, 188, 227, 131, 147, 227,
                130, 185, 229, 136, 169, 231, 148, 168, 232, 128, 133, 232, 168, 188, 230, 152, 142, 231, 148, 168, 49,
                51, 48, 49, 6, 3, 85, 4, 11, 12, 42, 229, 156, 176, 230, 150, 185, 229, 133, 172, 229, 133, 177, 229,
                155, 163, 228, 189, 147, 230, 131, 133, 229, 160, 177, 227, 130, 183, 227, 130, 185, 227, 131, 134,
                227, 131, 160, 230, 169, 159, 230, 167, 139, 48, 129, 187, 6, 3, 85, 29, 31, 4, 129, 179, 48, 129, 176,
                48, 129, 173, 160, 129, 170, 160, 129, 167, 164, 129, 164, 48, 129, 161, 49, 11, 48, 9, 6, 3, 85, 4, 6,
                19, 2, 74, 80, 49, 13, 48, 11, 6, 3, 85, 4, 10, 12, 4, 74, 80, 75, 73, 49, 37, 48, 35, 6, 3, 85, 4, 11,
                12, 28, 74, 80, 75, 73, 32, 102, 111, 114, 32, 117, 115, 101, 114, 32, 97, 117, 116, 104, 101, 110,
                116, 105, 99, 97, 116, 105, 111, 110, 49, 32, 48, 30, 6, 3, 85, 4, 11, 12, 23, 67, 82, 76, 32, 68, 105,
                115, 116, 114, 105, 98, 117, 116, 105, 111, 110, 32, 80, 111, 105, 110, 116, 115, 49, 21, 48, 19, 6, 3,
                85, 4, 11, 12, 12, 75, 97, 110, 97, 103, 97, 119, 97, 45, 107, 101, 110, 49, 35, 48, 33, 6, 3, 85, 4,
                3, 12, 26, 89, 111, 107, 111, 104, 97, 109, 97, 45, 115, 104, 105, 45, 78, 97, 107, 97, 45, 107, 117,
                32, 67, 82, 76, 68, 80, 48, 58, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 46, 48, 44, 48, 42, 6, 8, 43, 6, 1,
                5, 5, 7, 48, 1, 134, 30, 104, 116, 116, 112, 58, 47, 47, 111, 99, 115, 112, 97, 117, 116, 104, 110,
                111, 114, 109, 46, 106, 112, 107, 105, 46, 103, 111, 46, 106, 112, 48, 129, 178, 6, 3, 85, 29, 35, 4,
                129, 170, 48, 129, 167, 128, 20, 140, 213, 88, 106, 137, 20, 133, 229, 89, 55, 155, 126, 41, 212, 16,
                207, 210, 139, 53, 147, 161, 129, 136, 164, 129, 133, 48, 129, 130, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19,
                2, 74, 80, 49, 13, 48, 11, 6, 3, 85, 4, 10, 12, 4, 74, 80, 75, 73, 49, 37, 48, 35, 6, 3, 85, 4, 11, 12,
                28, 74, 80, 75, 73, 32, 102, 111, 114, 32, 117, 115, 101, 114, 32, 97, 117, 116, 104, 101, 110, 116,
                105, 99, 97, 116, 105, 111, 110, 49, 61, 48, 59, 6, 3, 85, 4, 11, 12, 52, 74, 97, 112, 97, 110, 32, 65,
                103, 101, 110, 99, 121, 32, 102, 111, 114, 32, 76, 111, 99, 97, 108, 32, 65, 117, 116, 104, 111, 114,
                105, 116, 121, 32, 73, 110, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 83, 121, 115, 116, 101,
                109, 115, 130, 4, 1, 51, 195, 73, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 2, 0, 136, 180, 211, 20, 167,
                117, 93, 40, 236, 27, 9, 158, 196, 94, 47, 238, 249, 146,
            ],
            public_objects: HashSet::new(),
        };
        circuit.public_objects.insert(4);

        let instance_columns: Vec<Vec<Fr>> = vec![
            (0..1 << super::Circuit::K)
                .map(|row_index| {
                    let row_value =
                        if let Some(_) = circuit.public_objects.get(&row_index) { row_index as u64 } else { 0u64 };
                    Fr::from(row_value)
                })
                .collect(),
            (0..1 << super::Circuit::K)
                .map(|row_index| {
                    let row_value =
                        if let Some(_) = circuit.public_objects.get(&row_index) { 0u64 } else { row_index as u64 };
                    Fr::from(row_value)
                })
                .collect(),
            std::iter::once(Fr::from(1u64 << 8))
                .chain((0..1 << super::Circuit::K).scan(super::State::new(), |state, row_index| {
                    let disclosure = if circuit.should_disclose(state.parsed_objects) {
                        if let Some(der_byte) = circuit.der_bytes.get(row_index) { *der_byte as u64 } else { 1u64 << 8 }
                    } else {
                        1u64 << 8
                    };

                    let action = if let Some(byte) = circuit.der_bytes.get(row_index) {
                        Action::Parse(*byte)
                    } else {
                        Action::DoNothing
                    };
                    *state = state.update(action);

                    Some(Fr::from(disclosure))
                }))
                .collect(),
        ];

        MockProver::run(super::Circuit::K as u32, &circuit, instance_columns)
            .expect("The circuit generation failed")
            .assert_satisfied();
    }
}
