const { readFileSync, read } = require('fs');
const { join } = require('path');
const sha256 = require('js-sha256');
const NodeRSA = require('node-rsa');
const CA_CERT_PATH = join(__dirname, '../../certs/ca_cert.pem');
const MYNA_CERT_PATH = join(__dirname, '../../certs/myna_cert.pem');

const ca_pem = readFileSync(CA_CERT_PATH, 'utf8');
const myna_pem = readFileSync(MYNA_CERT_PATH, 'utf8');
const ca_base64_string = ca_pem
    .replace('-----BEGIN CERTIFICATE-----', '')
    .replace('-----END CERTIFICATE-----', '')
    .replace(/\n/g, '');
const myna_base64_string = myna_pem
    .replace('-----BEGIN CERTIFICATE-----', '')
    .replace('-----END CERTIFICATE-----', '')
    .replace(/\n/g, '');
const ca_der = Buffer.from(ca_base64_string, 'base64');
const myna_der = Buffer.from(myna_base64_string, 'base64');

const modulus = '0x' + myna_der.slice(281, 281 + 256).toString('hex');
console.log("modulus", modulus);

function sign(key, buffer){
    var input = Buffer.from(buffer);
    var der = Buffer.from(key);
    const rsa = new NodeRSA(der, 'pkcs1-private-der');
    var sig = rsa.sign(input);
    return sig;
}
  
function verify(key, buffer, signature){
    var input = Buffer.from(buffer);
    var der = Buffer.from(key);
    var sig = Buffer.from(signature)
    const rsa = new NodeRSA(der, 'pkcs1-public-der');
    var result = rsa.verify(input, sig);
    return result;
}

const message = "hi, I'm nicoshark!";
console.log(sign(myna_der.slice(281, 281 + 256), message));