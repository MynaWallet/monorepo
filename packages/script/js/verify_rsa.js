const { readFileSync } = require('fs');
const { join } = require('path');
const sha256 = require('js-sha256');
const NodeRSA = require('node-rsa');
const CA_CERT_PATH = join(__dirname, '../../certs/ca_cert.pem');
const MYNA_CERT_PATH = join(__dirname, '../../certs/myna_cert.pem');
const ca_pem = readFileSync(CA_CERT_PATH, 'utf8');
const myna_pem = readFileSync(MYNA_CERT_PATH, 'utf8');

function bigint_to_array(n, k, x) {
  let mod = 1n;
  for (var idx = 0; idx < n; idx++) {
    mod = mod * 2n;
  }

  let ret = [];
  var x_temp = x;
  for (var idx = 0; idx < k; idx++) {
    ret.push(x_temp % mod);
    x_temp = x_temp / mod;
  }
  return ret;
}

const caBase64String = ca_pem
    .replace('-----BEGIN CERTIFICATE-----', '')
    .replace('-----END CERTIFICATE-----', '')
    .replace(/\n/g, '');
const mynaBase64String = myna_pem
    .replace('-----BEGIN CERTIFICATE-----', '')
    .replace('-----END CERTIFICATE-----', '')
    .replace(/\n/g, '');

const caDer = Buffer.from(caBase64String, 'base64');
const mynaDer = Buffer.from(mynaBase64String, 'base64');

const govSignatureHex = mynaDer.slice(1330, 1330  + 256).toString('hex');
const govModulusHex = caDer.slice(365, 365 + 256).toString('hex');
const modulusHex = mynaDer.slice(281, 281 + 256).toString('hex');
const signatureHex = "4e0e72adba23f1a132e1da737d00e65f01b93a524696cf4742691a2ee05df67dede82bb2846e0dfcb0e3eeee6bce87771374041202e0931302b5ba163bdc481dcdb740d407f50640889f9b19b39d869975fd4c30e67b6418377f2622b9755527dfc2d9ba8f284f5d3d8965d81ba8efccfed9686ccd85043a6625ca46465163446ca228ca4b843dd49a34fb8b77389012b0705be4b25c5fbf446c17728c693f56ebb73431e3d30148754596cd15b110d2aba831b92f36fcfbeddf008552fb3100aab375f204bec9cad71d780d1894b3460d0de555bc2cff3dc2e37f78c6b84eed1e16dca96096164e63258f2e96b70d47348b1e77c599c877ff084fd819d60237";
console.log("===== raw data =====");
console.log("govSignature", govSignatureHex);
console.log("govModulus", govModulusHex);
console.log("modulus", modulusHex);
console.log("signature", signatureHex);
// console.log("hashedTbs", bigint_to_array(121, 17, BigInt('0x' + hashedTbs)));


// certificate consists of
// - tbsCertificate 人によって長さが違う
// - signatureAlgorithm:  2 + 13 bytes // 固定
// - signature: 4 + 1 + 256 bytes // 固定

const cutSize = 2 + 13 + 4 + 1 + 256
const newSize = mynaDer.length - cutSize;
const tbsCert = mynaDer.slice(4, newSize);
console.log("========");
console.log("type of tbs cert", typeof(tbsCert[5]));
console.log("tbsCert length", tbsCert.length);
console.log(tbsCert[0]);
console.log(tbsCert[1305]);
console.log("========");

const govSignature = BigInt('0x' + govSignatureHex);
const govModulus = BigInt('0x' + govModulusHex);
const modulus = BigInt('0x' + modulusHex);
const signature = BigInt('0x' + signatureHex);

const hashedTbs = sha256(tbsCert);
console.log(hashedTbs);
const hashedMessage = "c329269d668612c42857cd89bc1e602693e064b23da2fdb4954f94a0843e771c";
const sha256Message = sha256(hashedMessage);
console.log(hashedMessage);
console.log(sha256Message);

console.log("===== Circuit input form =====");
console.log("govSignature", bigint_to_array(121, 17, govSignature));
console.log("govModulus", bigint_to_array(121, 17, govModulus));
console.log("modulus", bigint_to_array(121, 17, modulus));
console.log("signature", bigint_to_array(121, 17, signature));
console.log("hashedTbs", bigint_to_array(121, 17, BigInt('0x' + hashedTbs)));
console.log("hashedMessage", bigint_to_array(121, 17, BigInt('0x' + sha256Message)));
console.log("sha256Message", bigint_to_array(121, 17, BigInt("0x22bd41978f8e1d3ea52a4993021d71ede1b14c48e21b856ddf9dd67b32dbe0df")));

const e = "010001";

const key = new NodeRSA();
key.importKey({ n: Buffer.from(govModulusHex, 'hex'), e: Buffer.from(e, 'hex') }, 'components-public');
const isVerified = key.verify(Buffer.from(tbsCert, 'hex'), Buffer.from(govSignatureHex, 'hex'), 'buffer');
console.log(isVerified);

const key2 = new NodeRSA();
key2.importKey({ n: Buffer.from(modulusHex, 'hex'), e: Buffer.from(e, 'hex') }, 'components-public');
const isVerified2 = key2.verify(Buffer.from(hashedMessage, 'hex'), Buffer.from(signatureHex, 'hex'), 'buffer');
console.log(isVerified2);
