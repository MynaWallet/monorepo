const NodeRSA = require('node-rsa');
const crypto = require('crypto');

// RSA キーペアを生成
// const keyPair = new NodeRSA({b: 2048});

// RSA 秘密鍵からキーペアを生成
const keyPair = new NodeRSA(`-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAiplAiBRnkofh9YSdE9dv1tgFMmg7p4gQMWSmT7rXFM2lDd9p
AR08uFDIvqxNod+vYrJ3LCC/u6vi4tMPUMY2i9fX1YoaZKpwT2KXbuj+iZiM9lS/
GuHCMLdy+D35UsTvTjGz0/kqR7StimQnia6HIwL0CDjwFDg7R/xbRcR8iTFUOb1M
DSJnEKvbvIuH/3vIzKXne/xUdhTZdhTknnW1dPLn9CU9kiutP5/qRkh38rEgljmE
18F4ZxxdvLFU58/N98RFPJ8+vdLiG1KxmLzxOK7+Dwr48RlTfBWOuZXgCDSbzpPA
ZLBlpOtfww0QXsbYrJMk2pQ+5bnXTv4QZ0fiQQIDAQABAoIBADy5jKaSmhZ/ZqcV
pWTOHXg6SXyeYpSybrXVbXC9YgMBXvHHEtCkKLhw3KN+Br2HreZGXyR8bMUjOFM9
Ohaf4cI/nZc33PyCGJcPkV3Mw8kxh4Fd/CUhCN4jYkIVlSmIk3jlGe8j2jx0kxxx
aUlPqHHUl2sK11VklcSicrMMtau9xXPeCetONNCprL17cuzUERg9xjONZcksOVVJ
+wQ1TRsJoRSlCNcwNreJf7Y5NcsEG0Zsk3kRfFCcuZgq2lL3B2p/PMppDUv69mcp
DdoSnwFITS0yrQyrwzWxUqRd0kQ6lA9deQqO9EqiCarc2X03dJHk2nHL8fwQt3yx
Z3j6uMkCgYEAzls4qGUNhfig/KGesJ+6i5VoTYB0jrBrBZ/r6kKK2pxqFNjUUQ66
q8iFscVJGTZ55x/IkXD672Sk9J+Dzis7S39VouxDlLxCbHAbiwnbyoFqAXmWed21
WsTGmI6zSpwDkea0rtUCsyh9iBACOBvTntl9I3FLZwMwpybHreFFA2MCgYEAq/ER
Ib7eVUXq1aBoEWA6OSpXT1R+L3OWTEoafxds9MTlXdjysQb0IsssfIOOlrRsqVRr
w0vp2JtAT5b+0o5HWfZrm+7797RaiQOuExOcAsBnt6dOI8BF2yOlHjyRwACuR/5u
iIwhUuXKJq7LQZa6yudPvUXsEggioMh4QE1lXwsCgYBsSxETxlxvd0g87DBUbq6+
O/1N1uDUVR8FB6UN/jfSfA1rvVLG6xzps8T9wxQWiDE3KsXeFdWKtl2fButr0eI/
P6bYZncc4iNVtwutTtIqlvnjpkRi1ggrh2LVguXyfKee9NzTd3QAQ3qFYilX+rp2
/ZofdIBmohTxU3es97B5EwKBgG77lXFQwDYOgTwO492DXVGU0PJ1uVVrqHHpyxyJ
xFxh0yM2a8B0mpTyy+47BIimQRUzVrOihni4DVTSQ/0otUmOU3s9UQpcMawK4guS
NrtX8hqLNVbCPtTqNyRDOFjUl6oEwtgEi2X0yP0bCjt1zXA2yjODtJMXCTL68xOe
YXZtAoGBAMYYlTeRT8LwZ/k/NVG5/4k6UdT8hpGo8kdknwpgR1kQALr7Y4Iok9re
jbMqoWhZkiM7LeJTPlIOd8KswZ1LZnPMbkxufw/r2UBSTYGpACq9qFSDNOhcE9dF
S1Gpt7cNychauxmOm9hYqoAtmpTVBbkRtcn+lnntMuR7zS1bNsGA
-----END RSA PRIVATE KEY-----`)

 // RSA 秘密鍵を取得
 const privateKey = keyPair.exportKey('pkcs1-private');
 console.log("========== privateKey ==========");
 console.log(privateKey);
 
 // RSA 公開鍵を取得
 const publicKey = keyPair.exportKey('pkcs1-public');
 console.log("========== publicKey ==========");
 console.log(publicKey);

// modulus を取得
const components = keyPair.exportKey('components');
const modulus = components.n.toString('hex').replace('00', '');
console.log("========== modulus ==========");
console.log(BigInt("0x" + modulus));

// 署名対象データ
// const data = Buffer.from("42", 'utf-8');
const message = '42';
const data = Buffer.from("*", 'utf-8');
console.log(data);
// const data = "42";
const digest = crypto.createHash('sha256').update(data).digest();
console.log("========== digest ==========");
console.log(digest.toString('hex'));
console.log(BigInt("0x" + digest.toString('hex')));

// 署名
// keyPair.sign は内部的に sha256 して署名をしてくれるので渡すべき値に注意
// ライブラリの default のハッシュ関数が sha256 https://github.com/rzcoder/node-rsa/blob/master/src/schemes/pkcs1.js#L24
const signature = keyPair.sign(data, 'buffer', 'buffer');
console.log("========== signature ==========");
console.log(BigInt("0x" + signature.toString('hex')));

// 署名検証
const result = keyPair.verify(data, signature, 'buffer', 'buffer');
console.log("========== signature verification result ==========");
console.log(result);