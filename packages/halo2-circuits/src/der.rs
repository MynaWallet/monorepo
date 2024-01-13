use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Column, ConstraintSystem, Error, Expression, Fixed, Instance, Selector, TableColumn},
    poly::Rotation,
};
use std::collections::HashSet;
use zkevm_gadgets::util::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Action {
    Parse(u8),
    DoNothing,
}

impl Action {
    fn der_byte(self) -> u64 {
        if let Action::Parse(byte) = self { byte as u64 } else { 0 }
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
    parsed_types: u32,
    byte_kind: ByteKind,
}

impl State {
    fn new() -> State {
        State { header_size: 0, payload_size: 0, parsed_bytes: 0, parsed_types: 0, byte_kind: ByteKind::Type }
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

    // から文字列が来たらどうする
    // 来ない

    fn does_contain_der(self) -> bool {
        assert!(self.parsed_types > 0);
        Circuit::SHOULD_PARSE_PAYLOAD_AS_DER.get(self.parsed_types as usize - 1).copied().unwrap_or(false)
    }

    fn update(self, action: Action) -> State {
        println!("");
        if let Action::Parse(byte) = action {
            println!("der_byte: 0x{:0x}", byte);
        }
        println!("byte_kind: {:?}", self.byte_kind);
        println!("payload_size: 0x{:0x}", self.payload_size);
        println!("header_size: 0x{:0x}", self.header_size);
        println!("parsed_bytes: 0x{:0x}", self.parsed_bytes);
        println!("parsed_types: 0x{:0x}", self.parsed_types);

        let mut new_state = self.clone();
        new_state.parsed_bytes += 1;

        // parsed_bytes_next, header_size_next, payload_size_next are derived from is_type_cur, is_length_cur,
        // is_payload_cur, der_byte_cur, is_type_prev. In the constraint we use is_type_prev to check if this row is the
        // first row of the length bytes But since we can't read is_type_prev here we use parsed_bytes_cur == 1 to
        // check it

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

        // is_type_next, is_length_next, is_payload_next are derived from is_type_cur, is_length_cur, is_payload_cur,
        // parsed_bytes_next, payload_size_next, header_size_next.

        if self.byte_kind == ByteKind::Type {
            new_state.byte_kind = ByteKind::Length;
            new_state.parsed_bytes = 1;
            new_state.header_size = 2;
            new_state.payload_size = 0;
            new_state.parsed_types += 1;
        } else if self.byte_kind == ByteKind::Length && new_state.parsed_bytes == new_state.header_size {
            // TODO: HAndle the case of length 0 object.
            // This does not appear in MyNumber card so we can safely ignore it for now.
            println!("does_contain_der: {:?}", self.does_contain_der());
            println!("does_contain_der: {:?}", new_state.does_contain_der());
            if self.does_contain_der() {
                new_state.byte_kind = ByteKind::Type;
            } else {
                new_state.byte_kind = ByteKind::Payload;
            }
        } else if self.byte_kind == ByteKind::Payload
            && new_state.parsed_bytes == new_state.header_size + new_state.payload_size
        {
            new_state.byte_kind = ByteKind::Type;
        }

        new_state
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    // 0 for blinded rows. 1 otherwise.
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
    // How many bytes we have parsed in the object we're parsing.
    // When the object ends it circles back to 1.
    pub parsed_bytes: Column<Advice>,
    // How many type bytes we have parsed.
    pub parsed_types: Column<Advice>,
    pub does_contain_der: Column<Advice>,
    pub primitive_objects: TableColumn,
    pub constructed_objects: TableColumn,
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
    const BLINDING_FACTORS: usize = 6;
    const REDACTED: u64 = 1u64 << 8;

    // nth item of this list constrains whether or not we should parse the payload as DER, in the case of parsed_types =
    // n + 1. true if the object is NULL or has construted type
    // false otherwise
    const SHOULD_PARSE_PAYLOAD_AS_DER: [bool; 183] = [
        true,  // tbsCertificate
        true,  // tbsCertificate.version
        false, // tbsCertificate.version.
        false, // tbsCertificate.serialNumber
        true,  // tbsCertificate.signature
        false, // tbsCertificate.signature.algorithm
        true,  /* tbsCertificate.signature.parameters  This object has type NULL thus there's no payload. In this
                * case SHOULD_PARSE_PAYLOAD_AS_DER must be true because the byte after the end of the length bytes
                * is the beginning of next DER. Maybe we should handle this case explicitly? */
        true,  // tbsCertificate.issuer
        true,  // tbsCertificate.issuer[0]
        true,  // tbsCertificate.issuer[0][0]
        false, // tbsCertificate.issuer[0][0].type
        false, // tbsCertificate.issuer[0][0].value
        true,  // tbsCertificate.issuer[1]
        true,  // tbsCertificate.issuer[1][0]
        false, // tbsCertificate.issuer[1][0].type
        false, // tbsCertificate.issuer[1][0].value
        true,  // tbsCertificate.issuer[2]
        true,  // tbsCertificate.issuer[2][0]
        false, // tbsCertificate.issuer[2][0].type
        false, // tbsCertificate.issuer[2][0].value
        true,  // tbsCertificate.issuer[3]
        true,  // tbsCertificate.issuer[3][0]
        false, // tbsCertificate.issuer[3][0].type
        false, // tbsCertificate.issuer[3][0].value
        true,  // tbsCertificate.validity
        false, // tbsCertificate.validity.notBefore
        false, // tbsCertificate.validity.notAfter
        true,  // tbsCertificate.subject
        true,  // tbsCertificate.subject[0]
        true,  // tbsCertificate.subject[0][0]
        false, // tbsCertificate.subject[0][0].type
        false, // tbsCertificate.subject[0][0].value
        true,  // tbsCertificate.subject[1]
        true,  // tbsCertificate.subject[1][0]
        false, // tbsCertificate.subject[1][0].type
        false, // tbsCertificate.subject[1][0].value
        true,  // tbsCertificate.subject[2]
        true,  // tbsCertificate.subject[2][0]
        false, // tbsCertificate.subject[2][0].type
        false, // tbsCertificate.subject[2][0].value
        true,  // tbsCertificate.subject[3]
        true,  // tbsCertificate.subject[3][0]
        false, // tbsCertificate.subject[3][0].type
        false, // tbsCertificate.subject[3][0].value
        true,  // tbsCertificate.subjectPublicKeyInfo
        true,  // tbsCertificate.subjectPublicKeyInfo.algorithm
        false, // tbsCertificate.subjectPublicKeyInfo.algorithm.algorithm
        true,  // tbsCertificate.subjectPublicKeyInfo.algorithm.parameters NULL
        true,  // tbsCertificate.subjectPublicKeyInfo.subjectPublicKey BIT STRING
        true,  // tbsCertificate.subjectPublicKeyInfo.subjectPublicKey SEQUENCE
        false, // tbsCertificate.subjectPublicKeyInfo.subjectPublicKey[0]
        false, // tbsCertificate.subjectPublicKeyInfo.subjectPublicKey[1]
        true,  // tbsCertificate.extensions
        true,  // tbsCertificate.extensions SEQUENCE
        true,  // tbsCertificate.extensions.keyUsage
        false, // tbsCertificate.extensions.keyUsage.extnID
        false, // tbsCertificate.extensions.keyUsage.critical
        true,  // tbsCertificate.extensions.keyUsage.extnValue
        false, // tbsCertificate.extensions.keyUsage.extnValue BIT STRING
        true,  // tbsCertificate.extensions.subjectAltName
        false, // tbsCertificate.extensions.subjectAltName.extnID
        true,  // tbsCertificate.extensions.subjectAltName.extnValue OCTET STRING
        true,  // tbsCertificate.extensions.subjectAltName.extnValue SEQUENCE
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[0]
        false, // tbsCertificate.extensions.subjectAltName.extnValue[0].type
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[0].value constructed
        false, // tbsCertificate.extensions.subjectAltName.extnValue[0].value UTF8String
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[1]
        false, // tbsCertificate.extensions.subjectAltName.extnValue[1].type
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[1].value constructed
        false, // tbsCertificate.extensions.subjectAltName.extnValue[1].value UTF8String
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[2]
        false, // tbsCertificate.extensions.subjectAltName.extnValue[2].type
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[2].value constructed
        false, // tbsCertificate.extensions.subjectAltName.extnValue[2].value UTF8String
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[3]
        false, // tbsCertificate.extensions.subjectAltName.extnValue[3].type
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[3].value constructed
        false, // tbsCertificate.extensions.subjectAltName.extnValue[3].value UTF8String
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[4]
        false, // tbsCertificate.extensions.subjectAltName.extnValue[4].type
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[4].value constructed
        false, // tbsCertificate.extensions.subjectAltName.extnValue[4].value UTF8String
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[5]
        false, // tbsCertificate.extensions.subjectAltName.extnValue[5].type
        true,  // tbsCertificate.extensions.subjectAltName.extnValue[5].value constructed
        false, // tbsCertificate.extensions.subjectAltName.extnValue[5].value UTF8String
        true,  // tbsCertificate.extensions.certificatePolicies
        false, // tbsCertificate.extensions.certificatePolicies.extnID
        false, // tbsCertificate.extensions.certificatePolicies.critical
        true,  // tbsCertificate.extensions.certificatePolicies.extnValue OCTET STRING
        true,  // tbsCertificate.extensions.certificatePolicies.extnValue SEQUENCE
        true,  // tbsCertificate.extensions.certificatePolicies.extnValue SEQUENCE
        false, // tbsCertificate.extensions.certificatePolicies.extnValue OBJECT IDENTIFIER
        true,  // tbsCertificate.extensions.certificatePolicies.extnValue
        true,  // tbsCertificate.extensions.certificatePolicies.extnValue
        false, // tbsCertificate.extensions.certificatePolicies.extnValue
        false, // tbsCertificate.extensions.certificatePolicies.extnValue
        true,  // tbsCertificate.extensions.issuerAltName
        false, // tbsCertificate.extensions.issuerAltName.extnID
        true,  // tbsCertificate.extensions.issuerAltName.extnValue
        true,  // tbsCertificate.extensions.issuerAltName.extnValue
        true,  // tbsCertificate.extensions.issuerAltName.extnValue
        true,  // tbsCertificate.extensions.issuerAltName.extnValue
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[0]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[0]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[0]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[0]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[1]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[1]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[1]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[1]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[2]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[2]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[2]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[2]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[3]
        true,  // tbsCertificate.extensions.issuerAltName.extnValue[3]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[3]
        false, // tbsCertificate.extensions.issuerAltName.extnValue[3]
        true,  // tbsCertificate.extensions.cRLDistributionPoints
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnID
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[0]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[0]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[0]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[0]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[1]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[1]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[1]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[1]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[2]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[2]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[2]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[2]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[3]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[3]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[3]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[3]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[4]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[4]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[4]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[4]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[5]
        true,  // tbsCertificate.extensions.cRLDistributionPoints.extnValue[5]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[5]
        false, // tbsCertificate.extensions.cRLDistributionPoints.extnValue[5]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnID
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[0]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][0]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][0]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][0]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][0]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][1]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][1]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][1]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][1]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][2]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][2]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][2]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][2]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][3]
        true,  // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][3]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][3]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[1][3]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue[2]
        false, // tbsCertificate.extensions.authorityKeyIdentifier.extnValue
        true,  // tbsCertificate.extensions.subjectKeyIdentifier
        false, // tbsCertificate.extensions.subjectKeyIdentifier.extnID
        true,  // tbsCertificate.extensions.subjectKeyIdentifier.extnValue
        false, // tbsCertificate.extensions.subjectKeyIdentifier.extnValue
    ];
}

impl plonk::Circuit<Fr> for Circuit {
    type Config = Config;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        unreachable!();
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let config = Self::Config {
            is_enabled: meta.complex_selector(),
            der_byte: meta.advice_column(),
            is_type: meta.advice_column(),
            is_length: meta.advice_column(),
            is_payload: meta.advice_column(),
            header_size: meta.advice_column(),
            payload_size: meta.advice_column(),
            parsed_bytes: meta.advice_column(),
            parsed_types: meta.advice_column(),
            does_contain_der: meta.advice_column(),
            primitive_objects: meta.lookup_table_column(),
            constructed_objects: meta.lookup_table_column(),
            byte: meta.lookup_table_column(),
            is_below_128: meta.advice_column(),
            public_objects: meta.instance_column(),
            private_objects: meta.instance_column(),
            should_disclose: meta.advice_column(),
            disclosure: meta.instance_column(),
            constants: meta.fixed_column(),
        };
        meta.enable_equality(config.der_byte);
        meta.enable_equality(config.is_type);
        meta.enable_equality(config.is_length);
        meta.enable_equality(config.is_payload);
        meta.enable_equality(config.header_size);
        meta.enable_equality(config.payload_size);
        meta.enable_equality(config.parsed_bytes);
        meta.enable_equality(config.parsed_types);
        meta.enable_equality(config.does_contain_der);
        meta.enable_equality(config.is_below_128);
        meta.enable_equality(config.public_objects);
        meta.enable_equality(config.private_objects);
        meta.enable_equality(config.should_disclose);
        meta.enable_equality(config.constants);
        meta.enable_constant(config.constants);

        meta.create_gate("These columns must be either 0 or 1", |gate| {
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            let does_contain_der = gate.query_advice(config.does_contain_der, Rotation::cur());
            let is_below_128 = gate.query_advice(config.is_below_128, Rotation::cur());
            let should_disclose = gate.query_advice(config.should_disclose, Rotation::cur());
            vec![
                is_type.clone() * (is_type.clone() - Expression::Constant(Fr::one())),
                is_length.clone() * (is_length.clone() - Expression::Constant(Fr::one())),
                is_payload.clone() * (is_payload.clone() - Expression::Constant(Fr::one())),
                does_contain_der.clone() * (does_contain_der.clone() - Expression::Constant(Fr::one())),
                is_below_128.clone() * (is_below_128.clone() - Expression::Constant(Fr::one())),
                should_disclose.clone() * (should_disclose.clone() - Expression::Constant(Fr::one())),
            ]
        });

        meta.create_gate("If one of is_type, is_length, is_payload is on, the others should be off", |gate| {
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
            ]
        });

        meta.lookup("der_byte <= 0b11111111", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            vec![(is_enabled * der_byte, config.byte)]
        });

        meta.lookup("parsed_types must be in primitive_objects if does_contain_der is 0", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_types = region.query_advice(config.parsed_types, Rotation::cur());
            let does_contain_der = region.query_advice(config.does_contain_der, Rotation::cur());
            vec![(is_enabled * not::expr(does_contain_der) * parsed_types, config.primitive_objects)]
        });

        meta.lookup("parsed_types must be in constructed_objects if does_contain_der is 1", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_types = region.query_advice(config.parsed_types, Rotation::cur());
            let does_contain_der = region.query_advice(config.does_contain_der, Rotation::cur());
            vec![(is_enabled * does_contain_der * parsed_types, config.constructed_objects)]
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

        // assumption here is that parsed_types will never be 0.
        // If parsed_types became 0 both lookup would succeed,
        // and there would be no constraint on whether should_disclose should be 0 or 1.
        // Exception is that the first row where parsed_types is constrained to be 0.
        // The case of the first row is fine because is_payload is also constrained to be 0.
        // If is_payload = 0 no byte will be disclosed anyway.
        meta.lookup_any("parsed_types must be in public_objects if should_disclose is 1", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_types = region.query_advice(config.parsed_types, Rotation::cur());
            let public_objects = region.query_instance(config.public_objects, Rotation::cur());
            let should_disclose = region.query_advice(config.should_disclose, Rotation::cur());
            vec![(is_enabled * should_disclose * parsed_types, public_objects)]
        });

        meta.lookup_any("parsed_types must be in private_objects if should_disclose is 0", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_types = region.query_advice(config.parsed_types, Rotation::cur());
            let private_objects = region.query_instance(config.private_objects, Rotation::cur());
            let should_disclose = region.query_advice(config.should_disclose, Rotation::cur());
            let should_hide = Expression::Constant(Fr::one()) - should_disclose;
            vec![(is_enabled * should_hide * parsed_types, private_objects)]
        });

        meta.create_gate("Constrain order of operations", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let is_payload_cur = region.query_advice(config.is_payload, Rotation::cur());
            let is_payload_next = region.query_advice(config.is_payload, Rotation::next());
            let parsed_bytes_next = region.query_advice(config.parsed_bytes, Rotation::next());
            let payload_size_next = region.query_advice(config.payload_size, Rotation::next());
            let is_length_next = region.query_advice(config.is_length, Rotation::next());
            let is_length_cur = region.query_advice(config.is_length, Rotation::cur());
            let is_type_next = region.query_advice(config.is_type, Rotation::next());
            let is_type_cur = region.query_advice(config.is_type, Rotation::cur());
            let does_contain_der_next = region.query_advice(config.does_contain_der, Rotation::next());
            let header_size_next = region.query_advice(config.header_size, Rotation::next());
            vec![
                // If is_type is turned on, the previous is_type must be turned off.
                // Because a type tag consumes exactly 1 bit.
                is_enabled.clone() * is_type_next.clone() * is_type_cur.clone(),
                // If is_length is turned on, the previous is_payload must be turned off.
                // Because there's no such case that a length byte follows a payload byte.
                is_enabled.clone() * is_length_next.clone() * is_payload_cur.clone(),
                // If is_payload is turned on, the previous is_type must be turned off.
                // Because there's no such case that a payload byte follows a type byte.
                is_enabled.clone() * is_payload_next.clone() * is_type_cur.clone(),
                // If is_length_cur = 1 && is_length_next = 0 && does_contain_der_next = 0, then is_payload_next must
                // be 1 Because in a primitive object payload bytes follows length bytes
                is_enabled.clone()
                    * is_length_cur.clone()
                    * not::expr(is_length_next.clone())
                    * not::expr(does_contain_der_next.clone())
                    * not::expr(is_payload_next.clone()),
                // If is_length_cur = 1 && is_length_next = 0 && does_contain_der_next = 1, then is_type_next must be 1
                // Because in an object that's not primitive a type byte follows length bytes
                is_enabled.clone()
                    * is_length_cur.clone()
                    * not::expr(is_length_next.clone())
                    * does_contain_der_next.clone()
                    * not::expr(is_type_next.clone()),
                // If is_length_cur = 1 && is_length_next = 0, then parsed_bytes must equal header_size
                // This is to prevent an attacker from stop parsing length bytes early.
                is_enabled.clone()
                    * is_length_cur.clone()
                    * not::expr(is_length_next.clone())
                    * (parsed_bytes_next.clone() - header_size_next.clone()),
                // If is_payload_cur = 1 && is_payload_next = 0,
                // then parsed_bytes must equal header_size + payload_size.
                // This is to prevent an attacker from stop parsing payload bytes early.
                is_enabled.clone()
                    * is_payload_cur.clone()
                    * not::expr(is_payload_next.clone())
                    * (parsed_bytes_next.clone() - header_size_next.clone() - payload_size_next.clone()),
            ]
        });

        meta.create_gate("Constrain value of registers", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            // let disclosure_next = region.query_instance(config.disclosure, Rotation::next());
            let is_payload_cur = region.query_advice(config.is_payload, Rotation::cur());
            let parsed_bytes_next = region.query_advice(config.parsed_bytes, Rotation::next());
            let parsed_bytes_cur = region.query_advice(config.parsed_bytes, Rotation::cur());
            let parsed_types_next = region.query_advice(config.parsed_types, Rotation::next());
            let parsed_types_cur = region.query_advice(config.parsed_types, Rotation::cur());
            let header_size_next = region.query_advice(config.header_size, Rotation::next());
            let header_size_cur = region.query_advice(config.header_size, Rotation::cur());
            let payload_size_next = region.query_advice(config.payload_size, Rotation::next());
            let payload_size_cur = region.query_advice(config.payload_size, Rotation::cur());
            let is_length_cur = region.query_advice(config.is_length, Rotation::cur());
            let is_type_cur = region.query_advice(config.is_type, Rotation::cur());
            // We use is_type_prev only when is_length_cur is on.
            // Thus we will never use blinded is_type.
            let is_type_prev = region.query_advice(config.is_type, Rotation::prev());
            let is_below_128_cur = region.query_advice(config.is_below_128, Rotation::cur());
            let der_byte_cur = region.query_advice(config.der_byte, Rotation::cur());
            let should_disclose_cur = region.query_advice(config.should_disclose, Rotation::cur());
            vec![
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
                    * (parsed_types_next
                        - select::expr(
                            is_type_cur.clone(),
                            // If is_type = 1, increment the register.
                            parsed_types_cur.clone() + Expression::Constant(Fr::one()),
                            // otherwise, keep the value
                            parsed_types_cur.clone(),
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
                // is_enabled.clone()
                //     * (disclosure_next.clone()
                //         - select::expr( // If we should disclose the byte and::expr([should_disclose_cur.clone(),
                //           is_payload_cur.clone()]), // We disclose the byte der_byte_cur.clone(), // Otherwise we
                //           write a value that's over MAX_BYTE to indicate that those bytes are // redacted
                //           Expression::Constant(Fr::from(Self::REDACTED)),
                //         )),
            ]
        });

        config
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<Fr>) -> Result<(), Error> {
        layouter.assign_region(
            || "DerChip",
            |mut region| {
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
                    || "Assign parsed_types",
                    config.parsed_types,
                    0,
                    Fr::from(state.parsed_types as u64),
                )?;
                region.assign_advice_from_constant(
                    || "Assign does_contain_der",
                    config.does_contain_der,
                    0,
                    Fr::from(state.does_contain_der()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign should_disclose",
                    config.should_disclose,
                    0,
                    Fr::from(self.should_disclose(state.parsed_types)),
                )?;

                for row_index in 1..(1 << Self::K) - Self::BLINDING_FACTORS {
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
                    config.is_enabled.enable(&mut region, row_index - 1)?;

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
                        || "Assign parsed_types",
                        config.parsed_types,
                        row_index,
                        || Value::known(Fr::from(state.parsed_types as u64)),
                    )?;
                    region.assign_advice(
                        || "Assign does_contain_der",
                        config.does_contain_der,
                        row_index,
                        || Value::known(Fr::from(state.does_contain_der())),
                    )?;
                    region.assign_advice(
                        || "Assign should_disclose",
                        config.should_disclose,
                        row_index,
                        || Value::known(Fr::from(self.should_disclose(state.parsed_types))),
                    )?;
                }

                // Assign the last action.
                region.assign_advice_from_constant(
                    || "Assign der_byte",
                    config.der_byte,
                    (1 << Self::K) - Self::BLINDING_FACTORS - 1,
                    Fr::from(Action::DoNothing.der_byte()),
                )?;
                region.assign_advice_from_constant(
                    || "Assign is_below_128",
                    config.is_below_128,
                    (1 << Self::K) - Self::BLINDING_FACTORS - 1,
                    Fr::from(Action::DoNothing.is_below_128()),
                )?;

                Ok(())
            },
        )?;

        layouter.assign_table(
            || "DerChip",
            |mut table| {
                for row_index in 0..(1 << Self::K) - Self::BLINDING_FACTORS - 1 {
                    let row_value = row_index & 0b11111111;
                    table.assign_cell(
                        || "List of all possible bytes",
                        config.byte,
                        row_index,
                        || Value::known(Fr::from(row_value as u64)),
                    )?;

                    let row_value: u64 = match Self::SHOULD_PARSE_PAYLOAD_AS_DER.get(row_index) {
                        // While we're parsing the first object, parsed_types is 1.
                        // There will be always a 1 difference bitween the index of the object we're parsing and
                        // parsed_types.
                        Some(false) => row_index as u64 + 1,
                        Some(true) | None => 0,
                    };
                    table.assign_cell(
                        || "List of all objects of which contents are not serialized in DER",
                        config.primitive_objects,
                        row_index,
                        || Value::known(Fr::from(row_value)),
                    )?;

                    let row_value: u64 = match Self::SHOULD_PARSE_PAYLOAD_AS_DER.get(row_index) {
                        // If we're done parsing does_contain_der must be 0
                        Some(true) | None => row_index as u64 + 1,
                        Some(false) => 0,
                    };
                    table.assign_cell(
                        || "List of all objects of which contents are serialized in DER",
                        config.constructed_objects,
                        row_index,
                        || Value::known(Fr::from(row_value)),
                    )?;
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

impl Circuit {
    fn should_disclose(&self, parsed_types: u32) -> bool {
        self.public_objects.contains(&parsed_types)
    }
}

#[cfg(test)]
mod tests {
    use super::Action;
    use halo2_proofs::{dev::MockProver, halo2curves::bn256::Fr};
    use std::collections::HashSet;

    #[test]
    fn der() {
        let mut der_hex: String = std::env::var("SIGNER_CERTIFICATE")
            .expect("You must set $SIGNER_CERTIFICATE the certificate to use in the test");
        let mut der_bytes = hex::decode(der_hex).expect("Invalid $SIGNER_CERTIFICATE");
        der_bytes.extend(vec![0; (1 << super::Circuit::K) - super::Circuit::BLINDING_FACTORS - der_bytes.len()]);
        let circuit = super::Circuit { der_bytes, public_objects: HashSet::from_iter(vec![65]) };

        let instance_columns: Vec<Vec<Fr>> = vec![
            (0..(1 << super::Circuit::K) - super::Circuit::BLINDING_FACTORS)
                .map(|row_index| {
                    let row_value =
                        if circuit.public_objects.contains(&(row_index as u32)) { row_index as u64 } else { 0u64 };
                    Fr::from(row_value)
                })
                .collect(),
            (0..(1 << super::Circuit::K) - super::Circuit::BLINDING_FACTORS)
                .map(|row_index| {
                    let row_value =
                        if circuit.public_objects.contains(&(row_index as u32)) { 0u64 } else { row_index as u64 };
                    Fr::from(row_value)
                })
                .collect(),
            std::iter::once(Fr::from(super::Circuit::REDACTED))
                .chain((0..(1 << super::Circuit::K) - 1 - super::Circuit::BLINDING_FACTORS).scan(
                    super::State::new(),
                    |state, row_index| {
                        let action = if let Some(byte) = circuit.der_bytes.get(row_index) {
                            Action::Parse(*byte)
                        } else {
                            Action::DoNothing
                        };
                        let disclosure = if circuit.should_disclose(state.parsed_types) {
                            if let Action::Parse(byte) = action { byte as u64 } else { super::Circuit::REDACTED }
                        } else {
                            super::Circuit::REDACTED
                        };

                        *state = state.update(action);
                        Some(Fr::from(disclosure))
                    },
                ))
                .collect(),
        ];

        MockProver::run(super::Circuit::K as u32, &circuit, instance_columns)
            .expect("The circuit generation failed")
            .assert_satisfied();
    }
}
