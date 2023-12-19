import { Group } from "@semaphore-protocol/group"
import { poseidon1, poseidon2 } from "poseidon-lite";
import hash from "./semaphore/group/hash";
import fs from "fs";

async function calcTree(hash_list: any[], out: any[]) {
    if (hash_list.length == 1) {
        out.push(hash_list);
        return out;
    }

    if (hash_list.length % 2 == 1) {
        hash_list.push(hash_list[hash_list.length - 1]);
    }
    out.push(hash_list);
    let parent_hash_list:bigint[] = [];

    for (let i = 0; i < hash_list.length; i+=2) {
        parent_hash_list.push(poseidon2([hash_list[i], hash_list[i+1]]));
    }
    return calcTree(parent_hash_list, out);
}

async function main() {
    const group = new Group(1, 16);

    const userSecrets = [
        "0",
        "1",
        "2",
        "3"
    ];
    const modulus = [
        "10",
        "11",
        "12",
        "13"
    ];
    let hashedUserSecrets:bigint[] = [];
    for (let i = 0; i < userSecrets.length; i++) {
        const hashed = poseidon1([userSecrets[i]]);
        hashedUserSecrets.push(hashed);
    }

    let hashedModulus:bigint[] = [];
    for (let i = 0; i < modulus.length; i++) {
        const hashed = poseidon1([modulus[i]]);
        hashedModulus.push(hashed);
    }

    for (let i = 0; i < modulus.length; i++) {
        group.addMember(hashedModulus[i]);
        group.addMember(hashedUserSecrets[i]);
    }
    // console.log(group.members);
    // console.log(group.root);
    // console.log(group.zeroValue);
    console.log(group);
    // const zeroVal = hash(1);
    // console.log(zeroVal);

    // const hash_list = [];
    // for (let i = 0; i < hashedModulus.length; i++) {
    //     hash_list.push(hashedModulus[i]);
    //     hash_list.push(hashedUserSecrets[i]);
    // }

    // for (let i = hash_list.length; i < 65536; i++) {
    //     hash_list.push(zeroVal);
    // }

    // const tree = await calcTree(hash_list, []);
    // for (let i = 0; i < tree.length; i++) {
    //     for (let j = 0; j < tree[i].length; j++) {
    //       tree[i][j] = tree[i][j].toString();
    //     }
    // }
    // fs.writeFileSync("./json/tree.json", JSON.stringify(tree, null, 2));
    // console.log(tree[tree.length - 1][0]);

    const proof = await group.generateMerkleProof(0);
    console.log("===========================");
    console.log(proof);
}

main();