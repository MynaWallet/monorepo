const rsa = require ('js-x509-utils'); // for npm
const fs = require('fs');
const crypto = require('crypto');

const pem = fs.readFileSync('../../certs/myna_cert.pem', 'utf8')

const res = rsa.parse(pem, 'pem')

const messageBuffer = Buffer.from(res.tbsCertificate)
console.log("===== Message =====");
console.log(messageBuffer.toString('hex'))

const CApem = fs.readFileSync('../../certs/ca_cert.pem', 'utf8')
const caPubKey = crypto.createPublicKey(CApem)
console.log("===== ca pub key =====");
console.log(caPubKey);

const verify = crypto.createVerify('RSA-SHA256')
verify.update(messageBuffer)

const signature = Buffer.from(res.signatureValue)
console.log("===== signature =====");
console.log(signature.toString('hex'))

const result = verify.verify(caPubKey, signature)
console.log('result:', result ? 'OK' : 'NG')