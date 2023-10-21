const { readFileSync } = require("fs");
const { join } = require("path");

// Path to the PEM file containing the certificate
const PATH = join(__dirname, "../../halo2-circuits/certs/myna_cert.pem");

// Read the PEM file
const pem = readFileSync(PATH, "utf8");

// Remove the '-----BEGIN CERTIFICATE-----' and '-----END CERTIFICATE-----' lines
const base64String = pem
  .replace("-----BEGIN CERTIFICATE-----", "")
  .replace("-----END CERTIFICATE-----", "")
  .replace(/\n/g, "");

const der = Buffer.from(base64String, "base64");

// certificate consists of
// - tbsCertificate 人によって長さが違う
// - signatureAlgorithm:  2 + 13 bytes // 固定
// - signature: 4 + 1 + 256 bytes // 固定

const cutSize = 2 + 13 + 4 + 1 + 256;
const newSize = der.length - cutSize;

// 先頭4バイトは SEQUENCE の長さ
const tbsCertificate = der.slice(4, newSize);
console.log(tbsCertificate.toString("hex"));
