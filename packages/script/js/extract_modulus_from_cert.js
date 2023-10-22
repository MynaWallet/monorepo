const { readFileSync } = require('fs')
const { join } = require('path')

// Path to the PEM file containing the certificate
const PATH = join(__dirname, '../../halo2-circuits/certs/myna_cert.pem')

// Read the PEM file
const pem = readFileSync(PATH, 'utf8')

// Remove the '-----BEGIN CERTIFICATE-----' and '-----END CERTIFICATE-----' lines
const base64String = pem
    .replace('-----BEGIN CERTIFICATE-----', '')
    .replace('-----END CERTIFICATE-----', '')
    .replace(/\n/g, '')

const der = Buffer.from(base64String, 'base64')

// The modulus
console.log(`0x${der.slice(276 + 5, 276 + 5 + 256).toString('hex')}`)
