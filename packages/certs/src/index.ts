import forge from 'node-forge';
import fs from 'fs';
import { config } from 'process';

function main() {
    // === Generate Dummy Holder's Keys ===
    const holderKeys = forge.pki.rsa.generateKeyPair(2048);
    const holderSkPem = forge.pki.privateKeyToPem(holderKeys.privateKey);
    const holderPkPem = forge.pki.publicKeyToPem(holderKeys.publicKey);
    fs.writeFileSync('./pem/dummy_holder_keys/dummy_holder_sk.pem', holderSkPem, 'utf8');
    fs.writeFileSync('./pem/dummy_holder_keys/dummy_holder_pk.pem', holderPkPem, 'utf8');

    let cert = forge.pki.createCertificate();

    const configFile = JSON.parse(fs.readFileSync("./dummy_config/config.json", 'utf8'));

    cert.publicKey = holderKeys.publicKey;
    cert.serialNumber = configFile.serialNumber;
    cert.validity.notBefore = new Date(configFile.notBefore);
    cert.validity.notAfter = new Date(configFile.notAfter);

    const originalPem = fs.readFileSync('./pem/dummy_secret_cert.pem', 'utf8');
    const originalCert = forge.pki.certificateFromPem(originalPem);
    const originalIssuer = originalCert.issuer;

    const subjectAttr = [
        {
            type: '2.5.4.6',
            value: 'JP',
            valueTagClass: 19,
            name: 'countryName',
            shortName: 'C'
        },
        {
            type: '2.5.4.7',
            value: configFile.issuedPrefecture,
            valueTagClass: 12,
            name: 'localityName',
            shortName: 'L'
        },
        {
            type: '2.5.4.7',
            value: configFile.issuedCity,
            valueTagClass: 12,
            name: 'localityName',
            shortName: 'L'
        },
        {
            type: '2.5.4.3',
            value: configFile.issuedTiming,
            valueTagClass: 12,
            name: 'commonName',
            shortName: 'CN'
        }
    ]

    cert.setSubject(subjectAttr);
    cert.setIssuer(originalIssuer.attributes);

    // ======================= SET USER IDENTITY ===============================
    const identityArray = [
        [
            "1.2.392.200149.8.5.5.1",
            configFile.name
        ],
        [
            "1.2.392.200149.8.5.5.4",
            configFile.dateOfBirth
        ],
        [
            "1.2.392.200149.8.5.5.3",
            configFile.sex
        ],
        [
            "1.2.392.200149.8.5.5.5",
            configFile.address
        ],
        [
            "1.2.392.200149.8.5.5.2",
            configFile.altName
        ],
        [
            "1.2.392.200149.8.5.5.6",
            configFile.altAddress
        ],
    ];
    const sequenceValues = [];
    for (let i = 0; i < identityArray.length; i++) {
        const encoder = new TextEncoder();
        const utf8EncodedPayload = encoder.encode(identityArray[i][1]);
        const binaryString = String.fromCharCode(...utf8EncodedPayload);
        const asn1Value = forge.asn1.create(forge.asn1.Class.UNIVERSAL, forge.asn1.Type.UTF8, false, binaryString);
        const preSequence = forge.asn1.create(
            forge.asn1.Class.CONTEXT_SPECIFIC,
            0,
            true,
            [asn1Value]
        );
        const oidDer = forge.asn1.oidToDer(identityArray[i][0]).getBytes();
        const asn1Oid = forge.asn1.create(forge.asn1.Class.UNIVERSAL, forge.asn1.Type.OID, false, oidDer);

        // Wrap the sequence in an A0 context-specific tag
        const a0ExtensionValue = forge.asn1.create(forge.asn1.Class.CONTEXT_SPECIFIC, 0, true, [asn1Oid, preSequence]);
        sequenceValues.push(a0ExtensionValue); 
    }
    const sequence = forge.asn1.create(
        forge.asn1.Class.UNIVERSAL, forge.asn1.Type.SEQUENCE, true, sequenceValues);
    // =========================================================================


    const extensions = [
        {
            id: '2.5.29.15',
            critical: true,
            value: '\x03\x02\x06À',
            name: 'keyUsage',
            digitalSignature: true,
            nonRepudiation: true,
            keyEncipherment: false,
            dataEncipherment: false,
            keyAgreement: false,
            keyCertSign: false,
            cRLSign: false,
            encipherOnly: false,
            decipherOnly: false
        },
    ]

    cert.setExtensions(extensions);

    cert.extensions.push(
        {
            id: '2.5.29.17',
            critical: false,
            value: forge.asn1.toDer(sequence).getBytes(),
        }
    );
    cert.extensions.push(originalCert.extensions[2]);
    cert.extensions.push(originalCert.extensions[3]);

    // === Add Crl Data ===
    const crlConfig = [
        [
            "2.5.4.6 ",
            "JP"
        ],
        [
            "2.5.4.10",
            "JPKI"
        ],
        [
            "2.5.4.11",
            "JPKI for digital signature"
        ],
        [
            "2.5.4.11",
            "CRL Distribution Points"
        ],
        [
            "2.5.4.11",
            configFile.crlPrefecture
        ],
        [
            "2.5.4.3",
            configFile.crlCity
        ],
    ];
    const sequenceValues2 = [];
    for (let i = 0; i < crlConfig.length; i++) {
        const encoder = new TextEncoder();
        const utf8EncodedPayload = encoder.encode(crlConfig[i][1]);
        const binaryString = String.fromCharCode(...utf8EncodedPayload);
        let asn1Value;
        if (i == 0) {
            asn1Value =  forge.asn1.create(forge.asn1.Class.UNIVERSAL, forge.asn1.Type.PRINTABLESTRING, false, binaryString);
        } else {
            asn1Value = forge.asn1.create(forge.asn1.Class.UNIVERSAL, forge.asn1.Type.UTF8, false, binaryString);
        }
        const oidDer = forge.asn1.oidToDer(crlConfig[i][0]).getBytes();
        const asn1Oid = forge.asn1.create(forge.asn1.Class.UNIVERSAL, forge.asn1.Type.OID, false, oidDer);

        const preSequence = forge.asn1.create(
            forge.asn1.Class.UNIVERSAL,
            forge.asn1.Type.SEQUENCE,
            true,
            [asn1Oid, asn1Value]
        );
        const set = forge.asn1.create(
            forge.asn1.Class.UNIVERSAL,
            forge.asn1.Type.SET,
            true,
            [preSequence]
        );
        sequenceValues2.push(set); 
    }
    const sequence2 = forge.asn1.create(
        forge.asn1.Class.UNIVERSAL, forge.asn1.Type.SEQUENCE, true, sequenceValues2);

    const defined1 = forge.asn1.create(
        forge.asn1.Class.CONTEXT_SPECIFIC, 4, true, [sequence2]
    );
    const defined2 = forge.asn1.create(
        forge.asn1.Class.CONTEXT_SPECIFIC, 0, true, [defined1]
    );
    const defined3 = forge.asn1.create(
        forge.asn1.Class.CONTEXT_SPECIFIC, 0, true, [defined2]
    );
    const sequence3 = forge.asn1.create(
        forge.asn1.Class.UNIVERSAL,
        forge.asn1.Type.SEQUENCE,
        true,
        [defined3]
    );
    const sequence4 = forge.asn1.create(
        forge.asn1.Class.UNIVERSAL,
        forge.asn1.Type.SEQUENCE,
        true,
        [sequence3]
    );
    cert.extensions.push(
        {
            id: '2.5.29.31',
            critical: false,
            value: forge.asn1.toDer(sequence4).getBytes(),
        }
    );

    cert.extensions.push(originalCert.extensions[5]);
    cert.extensions.push(originalCert.extensions[6]);


    // === READ Dummy Gov Keys ===
    const govSkString = fs.readFileSync('./pem/dummy_gov_keys/dummy_gov_sk.pem', 'utf8');
    const govSk = forge.pki.privateKeyFromPem(govSkString);

    // === Give Signature to dummy cert ===
    cert.sign(govSk, forge.md.sha256.create());

    // Convert the certificate to PEM format
    const pem = forge.pki.certificateToPem(cert);
    console.log(pem);
    fs.writeFile("./pem/dummy_secret_cert.pem", pem, 'utf-8', (err) => {
        if (err) {
            console.error('Error writing to file:', err);
        } else {
            console.log('PEM file successfully saved.');
        }
    });
}

main();

