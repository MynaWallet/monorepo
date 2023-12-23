const { error } = require('console');
const fs = require('fs');
const { join } = require('path');

function main() {
    const INPUT_PATH = join(dirname, '../certs/secret_cert.pem');
    const OUTPUT_PATH = join(dirname, '../certs/selective_disclosure_input.json');

    const pem = fs.readFileSync(INPUT_PATH, 'utf8');

    const base64String = pem
        .replace('-----BEGIN CERTIFICATE-----', '')
        .replace('-----END CERTIFICATE-----', '')
        .replace(/\n/g, '')

    const der = Buffer.from(base64String, 'base64');

    const hexString = der.slice(4, 1589).toString('hex');
    let rawTbsArray = [];
    for (let i = 0; i < hexString.length; i += 2) {
        rawTbsArray.push(parseInt(hexString.substr(i, 2), 16));
    }
    for (let i = hexString.length / 2; i < 2048; i ++) {
        rawTbsArray.push(parseInt(0));
    }
    rawTbsArray[1585] = 128;
    rawTbsArray[1598] = 49;
    rawTbsArray[1599] = 136;

    let maskArray = [];
    for (let i = 0; i < 2048; i++) {
        if (i >= 161 && i <= 192) {
            maskArray.push(1);
        } else if (i >= 712 && i <= 853) {
            maskArray.push(1);
        } else if (i == 1585 || i == 1598 || i == 1599) {
            maskArray.push(1);
        } else {
            maskArray.push(0);
        }
    }

    let maskedArray = [];
    for (let i = 0; i < 2048; i++) {
        maskedArray[i] = rawTbsArray[i] * maskArray[i];
    }

    const output = {
        "rawTbsCert": rawTbsArray,
        "mask": maskArray,
        "maskedTbsCert": maskedArray
    }
    fs.writeFile(OUTPUT_PATH, JSON.stringify(output), (error) => {
        if (error) throw error;
    });
}

main();