import { Group } from "@semaphore-protocol/group"
import { poseidon1, poseidon2, poseidon16 } from "poseidon-lite";
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
    const group = new Group(42, 16);

    const userSecret = 42;
    const modulus = [
        "764015122569598935616865531886821953",
        "1712812914913423000341492655693458518",
        "2340392019621670749563502921178228467",
        "2097197962338670999293591473098186689",
        "1973549953448897839442749003933697107",
        "754777705836640329493068075616029734",
        "2452455626219750347491271955966891254",
        "173124902196474138485505111755323560",
        "1702999471315800896050648505035596391",
        "1023898167865931456226915211859860509",
        "986850648225152038852686132251061547",
        "1235059288812304588195509956254242542",
        "1272363995259886110571469295415785001",
        "566264394132604701522429791740303000",
        "1808607070958864568683440488451340977",
        "1161405003688027943932044419263143797",
        "2811114405719510898783780037530583"
    ];

    const hashedUserSecret = poseidon1([userSecret]);

    console.log("userSecret", hashedUserSecret);

    const intermediate = poseidon16([modulus[0], modulus[1], modulus[2], modulus[3], modulus[4], modulus[5], modulus[6], modulus[7], modulus[8], modulus[9], modulus[10], modulus[11], modulus[12], modulus[13], modulus[14], modulus[15]]);
    const hashedModulus = poseidon2([intermediate, modulus[16]]);
    const firstLeaf = poseidon2([hashedModulus, hashedUserSecret]);
    console.log("firstLeaf", firstLeaf);
    console.log(hashedModulus);
    group.addMember(hashedModulus);
    group.addMember(hashedUserSecret);
    console.log(group.members);
    console.log(group.root);
    console.log(group.zeroValue);
    // console.log(group);
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