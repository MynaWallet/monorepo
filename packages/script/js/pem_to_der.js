const fs = require('fs')
const { join } = require('path')

// Path to the PEM file containing the certificate
const INPUT_PATH = join(__dirname, '../../certs/secret_cert.pem');
const OUTPUT_PATH = join(__dirname, '../../certs/der.txt');

// Read the PEM file
const pem = fs.readFileSync(INPUT_PATH, 'utf8')

// Remove the '-----BEGIN CERTIFICATE-----' and '-----END CERTIFICATE-----' lines
const base64String = pem
    .replace('-----BEGIN CERTIFICATE-----', '')
    .replace('-----END CERTIFICATE-----', '')
    .replace(/\n/g, '')

const der = Buffer.from(base64String, 'base64')

// The modulus
console.log(`0x${der.toString('hex')}`);
fs.writeFile(OUTPUT_PATH, der.toString('hex'), (err) => {
 if (err) throw err;
});