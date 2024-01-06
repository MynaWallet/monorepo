use halo2_proofs::arithmetic::Field;
use std::collections::HashSet;

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Column, ConstraintSystem, Error, Expression, Fixed, Instance, Selector, TableColumn},
    poly::Rotation,
};
use zkevm_gadgets::util::*;
use std::cell::LazyCell;

const DOES_CONTAIN_DER: Vec<bool> = vec![
    true,  // SEQUENCE
    true,  // tbsCertificate
    false,  // tbsCertificate.version
    false,  // tbsCertificate.serialNumber
    true,  // tbsCertificate.signature
    false,  // tbsCertificate.signature.algorithm
    false,  // tbsCertificate.signature.parameters
    true, // tbsCertificate.issuer
    true, // tbsCertificate.issuer.countryName
    false,  // tbsCertificate.issuer.countryName.type
    false,  // tbsCertificate.issuer.countryName.value
    true,  // tbsCertificate.issuer.organizationName
    false,  // tbsCertificate.issuer.organizationName.type
    false,  // tbsCertificate.issuer.organizationName.value
    true,  // tbsCertificate.issuer.organizationalUnitName
    false,  // tbsCertificate.issuer.organizationalUnitName.type
    false,  // tbsCertificate.issuer.organizationalUnitName.value
    true,  // tbsCertificate.issuer.organizationalUnitName
    false,  // tbsCertificate.issuer.organizationalUnitName.type
    false,  // tbsCertificate.issuer.organizationalUnitName.value
    true,  // tbsCertificate.validity
    false,  // tbsCertificate.validity.notBefore
    false,  // tbsCertificate.validity.notAfter
    true,  // tbsCertificate.subject
    true,  // tbsCertificate.subject.countryName
    false,  // tbsCertificate.subject.countryName.type
    false,  // tbsCertificate.subject.countryName.value
    true,  // tbsCertificate.subject.localityName
    false,  // tbsCertificate.subject.localityName.type
    false,  // tbsCertificate.subject.localityName.value
    true,  // tbsCertificate.subject.localityName
    false,  // tbsCertificate.subject.localityName.type
    false,  // tbsCertificate.subject.localityName.value
    true,  // tbsCertificate.subject.commonName
    false,  // tbsCertificate.subject.commonName.type
    false,  // tbsCertificate.subject.commonName.value
    true,  // tbsCertificate.subjectPublicKeyInfo
    true,  // tbsCertificate.subjectPublicKeyInfo.algorithm
    false,  // tbsCertificate.subjectPublicKeyInfo.algorithm.algorithm
    false,  // tbsCertificate.subjectPublicKeyInfo.algorithm.parameters
    true,  // tbsCertificate.subjectPublicKeyInfo.subjectPublicKey
    true,  // tbsCertificate.subjectPublicKeyInfo.subjectPublicKey.E

];

const PRIMITIVE_OBJECTS: LazyCell<HashSet<u64>> = LazyCell::new(|| vec![
    2,  // version
    3,  // serialNumber
    5,  // signature.algorithm
    9,  // issuer.countryName.type
    10,  // issuer.countryNaemt.value
    12,  // issuer.organizationName.type
    13,  // issuer.organizationName.value
    15,  // issuer.organizationalUnitName.type
    16,  // issuer.organizationalUnitName.value
    18,  // issuer.organizationalUnitName.type
    19,  // issuer.organizationalUnitName.value
    21,  // validity.notBefore
    22,  // validity.notAfter
    25,  // subject.countryName.type
    26,  // subject.countryName.value
    28,  // localityName.type
    29,  // localityName.value
    31,  // localityName.type
    32,  // localityName.value
    34,  // commonName.type
    35,  // commonName.value
    38,  // subjectPublicKeyInfo.algorithm.algorithm
    42,  // subjectPublicKeyInfo.subjectPublicKey[0]
    43,  // subjectPublicKeyInfo.subjectPublicKey[1]
    46,  // extensions.authorityKeyIdentifier.extnID
    47,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.critical
    50,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.keyIdentifier
    54,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.countryName.type
    55,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.countryName.value
    57,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.organizationName.type
    58,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.organizationName.value
    60,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.organizationalUnitName.type
    61,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.organizationalUnitName.value
    63,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.organizationalUnitName.type
    64,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertIssuer.authorityCertIssuer.directoryName.organizationalUnitName.value
    66,  // extensions.authorityKeyIdentifier.extnValue.authorityKeyIdentifier.authorityCertSerialNumber
    68,  // extensions.keyUsage.extnID
    69,  // extensions.keyUsage.critical
    71,  // extensions.keyUsage.extnValue.keyUsage
    73,  // extensions.subjectAltName.extnID
    74,  // extensions.subjectAltName.critical
    78,  // extensions.subjectAltName.extnValue.otherName.commonName.type
    79,  // extensions.subjectAltName.extnValue.otherName.commonName.value
    82,  // extensions.subjectAltName.extnValue.otherName.dateOfBirth.type
    83,  // extensions.subjectAltName.extnValue.otherName.dateOfBirth.value
    86,  // extensions.subjectAltName.extnValue.otherName.gender.type
    87,  // extensions.subjectAltName.extnValue.otherName.gender.value
    90,  // extensions.subjectAltName.extnValue.otherName.address.type
    91,  // extensions.subjectAltName.extnValue.otherName.address.value
    94,  // extensions.subjectAltName.extnValue.otherName.substituteCharacterOfCommonName.type
    95,  // extensions.subjectAltName.extnValue.otherName.substituteCharacterOfCommonName.value
    98,  // extensions.subjectAltName.extnValue.otherName.substituteCharacterOfCommonName.type
    99,  // extensions.subjectAltName.extnValue.otherName.substituteCharacterOfCommonName.value
    101,  // extensions.issuerAltName.extnID
    102,  // extensions.issuerAltName.critical
    106,  // extensions.issuerAltName.extnValue.directoryName.countryName.type
    107,  // extensions.issuerAltName.extnValue.directoryName.countryName.value
    109,  // extensions.issuerAltName.extnValue.directoryName.organizationName.type
    110,  // extensions.issuerAltName.extnValue.directoryName.organizationName.value
    112,  // extensions.issuerAltName.extnValue.directoryName.organizationUnitName.type
    113,  // extensions.issuerAltName.extnValue.directoryName.organizationUnitName.value
    115,  // extensions.cRLDistributionPoints.extnID
    116,  // extensions.cRLDistributionPoints.critical
    122,  // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.countryName.type
    123,  // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.countryName.value
    125,  // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.organizationName.type
    126,  // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.organizationName.value
    128, // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.organizationUnitName.type
    129, // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.organizationUnitName.value
    131, // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.organizationUnitName.type
    132, // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.organizationUnitName.value
    134, // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.commonName.type
    135, // extensions.cRLDistributionPoints.extnValue.distributionPoint.fullName.directoryName.commonName.value
    137, // extensions.certificatePolicies.extnID
    138, // extensions.certificatePolicies.critical
    140, // extensions.certificatePolicies.extnValue.policyIdentifier
    142, // extensions.certificatePolicies.extnValue.policyQualifiers.policyQualifierId
    143, // extensions.certificatePolicies.extnValue.policyQualifiers.pqualifier
    145, // extensions.subjectKeyIdentifier.extnId
    146, // extensions.subjectKeyIdentifier.criticial
    149, // extensions.subjectKeyIdentifier.extnValue.subjectKeyIdentifier.keyIdentifie
r
].into_iter().collect());

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
    remaining_bytes: u8,
    byte_kind: ByteKind,
}

impl State {
    fn new() -> State {
        State {
            header_size: 0,
            payload_size: 0,
            parsed_bytes: 0,
            parsed_types: 0,
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
        Circuit::PRIMITIVE_OBJECTS.contains(self.parsed_types)
    }

    fn update(self, action: Action) -> State {
        println!("");
        if let Action::Parse(byte) = action {
            println!("der_byte: {:0x}", byte);
        }
        println!("byte_kind: {:?}", self.byte_kind);
        println!("payload_size: {:0x}", self.payload_size);
        println!("header_size: {}", self.header_size);
        println!("parsed_bytes: {}", self.parsed_bytes);
        println!("parsed_types: {}", self.parsed_types);

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
            new_state.type_tag = match action {
                Action::Parse(byte) => byte,
                Action::DoNothing => 0,
            };
            new_state.parsed_bytes = 1;
            new_state.header_size = 2;
            new_state.payload_size = 0;
            new_state.parsed_types += 1;
        } else if self.byte_kind == ByteKind::Length && new_state.parsed_bytes == new_state.header_size {
            if self.is_primitive() {
                new_state.byte_kind = ByteKind::Payload;
            } else {
                new_state.byte_kind = ByteKind::Type;
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
    // How many bytes we have left to parse.
    pub remaining_bytes: Column<Advice>,
    pub is_primitive: Column<Advice>,
    pub primitive_objects: TableColumn,
    pub constructed_objects: TableColumn,
    // 0..2^(K-1)
    pub range_check: TableColumn,
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
    const PUBLIC_OBJECTS: HashSet<u32> = {

    }
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
            remaining_bytes: meta.advice_column(),
            is_primitive: meta.advice_column(),
            primitive_objects: meta.lookup_table_column(),
            constructed_objects: meta.lookup_table_column(),
            range_check: meta.lookup_table_column(),
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
        meta.enable_equality(config.remaining_bytes);
        meta.enable_equality(config.is_primitive);
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

        meta.create_gate("If all of is_type, is_length, is_payload is off, remaining_bytes = 0", |gate| {
            let is_enabled = gate.query_selector(config.is_enabled);
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            let remaining_bytes = gate.query_advice(config.remaining_bytes, Rotation::cur());
            vec![is_enabled * and::expr([not::expr(is_type), not::expr(is_length), not::expr(is_payload)]) * remaining_bytes]
        });

        meta.create_gate("If one of is_type, is_length, is_payload is on, remaining_bytes should decrement", |gate| {
            let is_enabled = gate.query_selector(config.is_enabled);
            let is_type = gate.query_advice(config.is_type, Rotation::cur());
            let is_length = gate.query_advice(config.is_length, Rotation::cur());
            let is_payload = gate.query_advice(config.is_payload, Rotation::cur());
            let remaining_bytes_cur = gate.query_advice(config.remaining_bytes, Rotation::cur());
            let remaining_bytes_next = gate.query_advice(config.remaining_bytes, Rotation::next());
            vec![is_enabled * or::expr([is_type, is_length, is_payload]) * (remaining_bytes_cur - remaining_bytes_next + Expression::Constant(Fr::one()))]
        });

        meta.lookup("remaining_bytes < 2^(K-1)", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let remaining_bytes = region.query_advice(config.remaining_bytes, Rotation::cur());
            vec![(is_enabled * remaining_bytes, config.range_check)]
        });

        meta.lookup("der_byte <= 0b11111111", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let der_byte = region.query_advice(config.der_byte, Rotation::cur());
            let left_shift = Expression::Constant(Fr::from(1 << (Self::K - 1 - 8)));
            vec![(is_enabled * der_byte * left_shift, config.range_check)]
        });

        meta.lookup("parsed_types must be in primitive_objects if is_primitive is 1", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_types = region.query_advice(config.parsed_types, Rotation::cur());
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            vec![(is_enabled * is_primitive * parsed_types, config.primitive_objects)]
        });

        meta.lookup("parsed_types must be in constructed_objects if is_primitive is 0", |region| {
            let is_enabled = region.query_selector(config.is_enabled);
            let parsed_types = region.query_advice(config.parsed_types, Rotation::cur());
            let is_primitive = region.query_advice(config.is_primitive, Rotation::cur());
            let is_constructed = Expression::Constant(Fr::one()) - is_primitive;
            vec![(is_enabled * not::expr(is_primitive) * parsed_types, config.constructed_objects)]
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
            let is_primitive_next = region.query_advice(config.is_primitive, Rotation::next());
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
                // If is_length_cur = 1 && is_length_next = 0 && is_primitive_next = 1, then is_payload_next must be 1
                // Because in a primitive object payload bytes follows length bytes
                is_enabled.clone()
                    * is_length_cur.clone()
                    * not::expr(is_length_next.clone())
                    * is_primitive_next.clone()
                    * not::expr(is_payload_next.clone()),
                // If is_length_cur = 1 && is_length_next = 0 && is_primitive_next = 0, then is_type_next must be 1
                // Because in an object that's not primitive a type byte follows length bytes
                is_enabled.clone()
                    * is_length_cur.clone()
                    * not::expr(is_length_next.clone())
                    * not::expr(is_primitive_next.clone())
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
            let is_type_prev = region.query_advice(config.is_type, Rotation::prev());
            let is_below_128_cur = region.query_advice(config.is_below_128, Rotation::cur());
            let der_byte_cur = region.query_advice(config.der_byte, Rotation::cur());
            let should_disclose_cur = region.query_advice(config.should_disclose, Rotation::cur());
            vec![
                is_enabled.clone() * or::expr([is_type_cur.clone(), is_length_cur.clone(), is_payload_cur.clone()])
                    * (parsed_bytes_next
                        - select::expr(
                            is_type_cur.clone(),
                            // If is_type = 1, reset the register with 1.
                            Expression::Constant(Fr::one()),
                            // otherwise, increment the register
                            parsed_bytes_cur.clone() + Expression::Constant(Fr::one()),
                        )),
                is_enabled.clone() * or::expr([is_type_cur.clone(), is_length_cur.clone(), is_payload_cur.clone()])
                    * (parsed_types_next
                        - select::expr(
                            is_type_cur.clone(),
                            // If is_type = 1, increment the register.
                            parsed_types_cur.clone() + Expression::Constant(Fr::one()),
                            // otherwise, keep the value
                            parsed_types_cur.clone(),
                        )),
                is_enabled.clone() * or::expr([is_type_cur.clone(), is_length_cur.clone(), is_payload_cur.clone()])
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
                is_enabled.clone() * or::expr([is_type_cur.clone(), is_length_cur.clone(), is_payload_cur.clone()])
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
                    || "Assign is_primitive",
                    config.is_primitive,
                    0,
                    Fr::from(state.is_primitive()),
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
                        || "Assign is_primitive",
                        config.is_primitive,
                        row_index,
                        || Value::known(Fr::from(state.is_primitive())),
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

                    let row_value: u64 = if Self::PRIMITIVE_OBJECTS.contains(row_index) { row_index as u64 } else { 0 };
                    table.assign_cell(
                        || "List of all objects of which contents are not serialized in DER",
                        config.primitive_objects,
                        row_index,
                        || Value::known(Fr::from(row_value)),
                    )?;

                    let row_value: u64 = if Self::PRIMITIVE_OBJECTS.contains(row_index) { 0 } else { row_index as u64 };
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
        let mut der_bytes = vec![
            0x30, 0x82, 0x5, 0x16, 0xA0, 0x3, 0x2, 0x1, 0x2, 0x2, 0x4, 0x1, 0x8C, 0x45, 0x3E, 0x30, 0xD, 0x6, 0x9,
            0x2A, 0x86, 0x48, 0x86, 0xF7, 0xD, 0x1, 0x1, 0xB, 0x5, 0x0, 0x30, 0x81, 0x82, 0x31, 0xB, 0x30, 0x9, 0x6,
            0x3, 0x55, 0x4, 0x6, 0x13, 0x2, 0x4A, 0x50, 0x31, 0xD, 0x30, 0xB, 0x6, 0x3, 0x55, 0x4, 0xA, 0xC, 0x4, 0x4A,
            0x50, 0x4B, 0x49, 0x31, 0x25, 0x30, 0x23, 0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x1C, 0x4A, 0x50, 0x4B, 0x49,
            0x20, 0x66, 0x6F, 0x72, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6E, 0x74, 0x69,
            0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x31, 0x3D, 0x30, 0x3B, 0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x34, 0x4A,
            0x61, 0x70, 0x61, 0x6E, 0x20, 0x41, 0x67, 0x65, 0x6E, 0x63, 0x79, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x4C, 0x6F,
            0x63, 0x61, 0x6C, 0x20, 0x41, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x69, 0x74, 0x79, 0x20, 0x49, 0x6E, 0x66, 0x6F,
            0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x73, 0x30, 0x1E, 0x17,
            0xD, 0x32, 0x30, 0x30, 0x35, 0x31, 0x39, 0x31, 0x37, 0x33, 0x38, 0x35, 0x35, 0x5A, 0x17, 0xD, 0x32, 0x34,
            0x31, 0x32, 0x30, 0x37, 0x31, 0x34, 0x35, 0x39, 0x35, 0x39, 0x5A, 0x30, 0x2F, 0x31, 0xB, 0x30, 0x9, 0x6,
            0x3, 0x55, 0x4, 0x6, 0x13, 0x2, 0x4A, 0x50, 0x31, 0x20, 0x30, 0x1E, 0x6, 0x3, 0x55, 0x4, 0x3, 0xC, 0x17,
            0x33, 0x30, 0x38, 0x30, 0x31, 0x34, 0x45, 0x34, 0x35, 0x4A, 0x45, 0x46, 0x49, 0x47, 0x31, 0x34, 0x31, 0x30,
            0x34, 0x30, 0x30, 0x33, 0x41, 0x30, 0x82, 0x1, 0x22, 0x30, 0xD, 0x6, 0x9, 0x2A, 0x86, 0x48, 0x86, 0xF7,
            0xD, 0x1, 0x1, 0x1, 0x5, 0x0, 0x3, 0x82, 0x1, 0xF, 0x0, 0x30, 0x82, 0x1, 0xA, 0x2, 0x82, 0x1, 0x1, 0x0,
            0xB2, 0x37, 0xE1, 0xD, 0xE3, 0xAB, 0xAF, 0x29, 0xAE, 0xCA, 0x23, 0xC2, 0xA, 0x55, 0x2B, 0xD3, 0x25, 0x14,
            0x6C, 0x7, 0x56, 0xFC, 0x5B, 0x9, 0x7B, 0x99, 0x1E, 0x7E, 0xC5, 0x69, 0x55, 0x1E, 0xBB, 0x1D, 0xB9, 0x7B,
            0x71, 0x3D, 0x7C, 0x66, 0x62, 0x4D, 0x78, 0xAF, 0xF9, 0x6, 0x8C, 0x12, 0xE4, 0x41, 0xDD, 0xD1, 0x85, 0x21,
            0xA4, 0x2F, 0x69, 0xD8, 0x5C, 0x9A, 0xC, 0xEE, 0x70, 0x7C, 0x92, 0x6D, 0x2, 0xEA, 0x7, 0x58, 0x8D, 0x39,
            0x17, 0x33, 0x4, 0xD9, 0x1D, 0xE3, 0x1, 0x73, 0x2E, 0xAF, 0xBA, 0x90, 0xCB, 0xD7, 0xA0, 0xA4, 0x16, 0xB3,
            0x3A, 0x54, 0x4A, 0x15, 0x95, 0x30, 0xA9, 0x1A, 0xA6, 0xB8, 0xC5, 0x2E, 0x4C, 0x60, 0xEA, 0x51, 0x20, 0x76,
            0xB4, 0xCA, 0x8F, 0x9D, 0x61, 0x53, 0x2A, 0xFE, 0x9A, 0x61, 0x24, 0xD4, 0xBB, 0x86, 0xDA, 0xC4, 0x9A, 0xC2,
            0x69, 0x4B, 0xA3, 0xE4, 0x29, 0x73, 0xBE, 0x8B, 0x1, 0x74, 0x50, 0xA7, 0xF1, 0xA4, 0x9A, 0x64, 0x56, 0x4E,
            0x71, 0x34, 0x82, 0xD7, 0xD3, 0x2D, 0x4, 0xFB, 0x45, 0xF5, 0x69, 0xD9, 0x86, 0xD6, 0xF1, 0xCC, 0xC4, 0x4B,
            0x7B, 0x71, 0xB2, 0x7B, 0x5D, 0x86, 0x41, 0x15, 0xC, 0x11, 0x6F, 0x85, 0xAA, 0x11, 0x5D, 0xCE, 0xDC, 0xE7,
            0xA5, 0xF0, 0xC6, 0x5F, 0x4, 0x8E, 0xCB, 0xD3, 0xA6, 0x54, 0xC2, 0xFF, 0xE5, 0x14, 0xEC, 0xA3, 0x52, 0x60,
            0x27, 0xD1, 0x7F, 0x12, 0xCF, 0x4, 0x4B, 0x92, 0xCA, 0x73, 0xA5, 0x41, 0xA2, 0x20, 0xA1, 0xBE, 0xA3, 0xF9,
            0x2D, 0xC4, 0x52, 0x1, 0x52, 0x3B, 0xB2, 0xAF, 0x12, 0x89, 0x4C, 0xC9, 0x81, 0xB1, 0x5D, 0xB1, 0x9F, 0x46,
            0xAA, 0x28, 0x6C, 0x1D, 0x68, 0x3A, 0x2D, 0x43, 0x81, 0x31, 0x56, 0x9E, 0x9F, 0xFA, 0x14, 0xCD, 0x37, 0xD5,
            0xF5, 0xD4, 0x61, 0x37, 0x2, 0x3, 0x1, 0x0, 0x1, 0xA3, 0x82, 0x2, 0xFC, 0x30, 0x82, 0x2, 0xF8, 0x30, 0xE,
            0x6, 0x3, 0x55, 0x1D, 0xF, 0x1, 0x1, 0xFF, 0x4, 0x4, 0x3, 0x2, 0x7, 0x80, 0x30, 0x13, 0x6, 0x3, 0x55, 0x1D,
            0x25, 0x4, 0xC, 0x30, 0xA, 0x6, 0x8, 0x2B, 0x6, 0x1, 0x5, 0x5, 0x7, 0x3, 0x2, 0x30, 0x49, 0x6, 0x3, 0x55,
            0x1D, 0x20, 0x1, 0x1, 0xFF, 0x4, 0x3F, 0x30, 0x3D, 0x30, 0x3B, 0x6, 0xB, 0x2A, 0x83, 0x8, 0x8C, 0x9B, 0x55,
            0x8, 0x5, 0x1, 0x3, 0x1E, 0x30, 0x2C, 0x30, 0x2A, 0x6, 0x8, 0x2B, 0x6, 0x1, 0x5, 0x5, 0x7, 0x2, 0x1, 0x16,
            0x1E, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x6A, 0x70, 0x6B, 0x69, 0x2E, 0x67,
            0x6F, 0x2E, 0x6A, 0x70, 0x2F, 0x63, 0x70, 0x73, 0x2E, 0x68, 0x74, 0x6D, 0x6C, 0x30, 0x81, 0xB7, 0x6, 0x3,
            0x55, 0x1D, 0x12, 0x4, 0x81, 0xAF, 0x30, 0x81, 0xAC, 0xA4, 0x81, 0xA9, 0x30, 0x81, 0xA6, 0x31, 0xB, 0x30,
            0x9, 0x6, 0x3, 0x55, 0x4, 0x6, 0x13, 0x2, 0x4A, 0x50, 0x31, 0x27, 0x30, 0x25, 0x6, 0x3, 0x55, 0x4, 0xA,
            0xC, 0x1E, 0xE5, 0x85, 0xAC, 0xE7, 0x9A, 0x84, 0xE5, 0x80, 0x8B, 0xE4, 0xBA, 0xBA, 0xE8, 0xAA, 0x8D, 0xE8,
            0xA8, 0xBC, 0xE3, 0x82, 0xB5, 0xE3, 0x83, 0xBC, 0xE3, 0x83, 0x93, 0xE3, 0x82, 0xB9, 0x31, 0x39, 0x30, 0x37,
            0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x30, 0xE5, 0x85, 0xAC, 0xE7, 0x9A, 0x84, 0xE5, 0x80, 0x8B, 0xE4, 0xBA,
            0xBA, 0xE8, 0xAA, 0x8D, 0xE8, 0xA8, 0xBC, 0xE3, 0x82, 0xB5, 0xE3, 0x83, 0xBC, 0xE3, 0x83, 0x93, 0xE3, 0x82,
            0xB9, 0xE5, 0x88, 0xA9, 0xE7, 0x94, 0xA8, 0xE8, 0x80, 0x85, 0xE8, 0xA8, 0xBC, 0xE6, 0x98, 0x8E, 0xE7, 0x94,
            0xA8, 0x31, 0x33, 0x30, 0x31, 0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x2A, 0xE5, 0x9C, 0xB0, 0xE6, 0x96, 0xB9,
            0xE5, 0x85, 0xAC, 0xE5, 0x85, 0xB1, 0xE5, 0x9B, 0xA3, 0xE4, 0xBD, 0x93, 0xE6, 0x83, 0x85, 0xE5, 0xA0, 0xB1,
            0xE3, 0x82, 0xB7, 0xE3, 0x82, 0xB9, 0xE3, 0x83, 0x86, 0xE3, 0x83, 0xA0, 0xE6, 0xA9, 0x9F, 0xE6, 0xA7, 0x8B,
            0x30, 0x81, 0xBB, 0x6, 0x3, 0x55, 0x1D, 0x1F, 0x4, 0x81, 0xB3, 0x30, 0x81, 0xB0, 0x30, 0x81, 0xAD, 0xA0,
            0x81, 0xAA, 0xA0, 0x81, 0xA7, 0xA4, 0x81, 0xA4, 0x30, 0x81, 0xA1, 0x31, 0xB, 0x30, 0x9, 0x6, 0x3, 0x55,
            0x4, 0x6, 0x13, 0x2, 0x4A, 0x50, 0x31, 0xD, 0x30, 0xB, 0x6, 0x3, 0x55, 0x4, 0xA, 0xC, 0x4, 0x4A, 0x50,
            0x4B, 0x49, 0x31, 0x25, 0x30, 0x23, 0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x1C, 0x4A, 0x50, 0x4B, 0x49, 0x20,
            0x66, 0x6F, 0x72, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6E, 0x74, 0x69, 0x63,
            0x61, 0x74, 0x69, 0x6F, 0x6E, 0x31, 0x20, 0x30, 0x1E, 0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x17, 0x43, 0x52,
            0x4C, 0x20, 0x44, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x50, 0x6F, 0x69,
            0x6E, 0x74, 0x73, 0x31, 0x15, 0x30, 0x13, 0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0xC, 0x4B, 0x61, 0x6E, 0x61, 0x67,
            0x61, 0x77, 0x61, 0x2D, 0x6B, 0x65, 0x6E, 0x31, 0x23, 0x30, 0x21, 0x6, 0x3, 0x55, 0x4, 0x3, 0xC, 0x1A,
            0x59, 0x6F, 0x6B, 0x6F, 0x68, 0x61, 0x6D, 0x61, 0x2D, 0x73, 0x68, 0x69, 0x2D, 0x4E, 0x61, 0x6B, 0x61, 0x2D,
            0x6B, 0x75, 0x20, 0x43, 0x52, 0x4C, 0x44, 0x50, 0x30, 0x3A, 0x6, 0x8, 0x2B, 0x6, 0x1, 0x5, 0x5, 0x7, 0x1,
            0x1, 0x4, 0x2E, 0x30, 0x2C, 0x30, 0x2A, 0x6, 0x8, 0x2B, 0x6, 0x1, 0x5, 0x5, 0x7, 0x30, 0x1, 0x86, 0x1E,
            0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x6F, 0x63, 0x73, 0x70, 0x61, 0x75, 0x74, 0x68, 0x6E, 0x6F, 0x72,
            0x6D, 0x2E, 0x6A, 0x70, 0x6B, 0x69, 0x2E, 0x67, 0x6F, 0x2E, 0x6A, 0x70, 0x30, 0x81, 0xB2, 0x6, 0x3, 0x55,
            0x1D, 0x23, 0x4, 0x81, 0xAA, 0x30, 0x81, 0xA7, 0x80, 0x14, 0x8C, 0xD5, 0x58, 0x6A, 0x89, 0x14, 0x85, 0xE5,
            0x59, 0x37, 0x9B, 0x7E, 0x29, 0xD4, 0x10, 0xCF, 0xD2, 0x8B, 0x35, 0x93, 0xA1, 0x81, 0x88, 0xA4, 0x81, 0x85,
            0x30, 0x81, 0x82, 0x31, 0xB, 0x30, 0x9, 0x6, 0x3, 0x55, 0x4, 0x6, 0x13, 0x2, 0x4A, 0x50, 0x31, 0xD, 0x30,
            0xB, 0x6, 0x3, 0x55, 0x4, 0xA, 0xC, 0x4, 0x4A, 0x50, 0x4B, 0x49, 0x31, 0x25, 0x30, 0x23, 0x6, 0x3, 0x55,
            0x4, 0xB, 0xC, 0x1C, 0x4A, 0x50, 0x4B, 0x49, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20,
            0x61, 0x75, 0x74, 0x68, 0x65, 0x6E, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x31, 0x3D, 0x30, 0x3B,
            0x6, 0x3, 0x55, 0x4, 0xB, 0xC, 0x34, 0x4A, 0x61, 0x70, 0x61, 0x6E, 0x20, 0x41, 0x67, 0x65, 0x6E, 0x63,
            0x79, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x41, 0x75, 0x74, 0x68, 0x6F, 0x72,
            0x69, 0x74, 0x79, 0x20, 0x49, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x53, 0x79,
            0x73, 0x74, 0x65, 0x6D, 0x73, 0x82, 0x4, 0x1, 0x33, 0xC3, 0x49, 0x30, 0x1D, 0x6, 0x3, 0x55, 0x1D, 0xE, 0x4,
            0x16, 0x4, 0x14, 0x2, 0x0, 0x88, 0xB4, 0xD3, 0x14, 0xA7, 0x75, 0x5D, 0x28, 0xEC, 0x1B, 0x9, 0x9E, 0xC4,
            0x5E, 0x2F, 0xEE, 0xF9, 0x92,
        ];
        der_bytes.extend(vec![0; (1 << super::Circuit::K) - super::Circuit::BLINDING_FACTORS - der_bytes.len()]);
        let circuit = super::Circuit { der_bytes, public_objects: HashSet::from_iter(vec![3]) };

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
