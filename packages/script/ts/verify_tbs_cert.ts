import { ethers } from 'ethers'
import { join } from 'path'
import NodeRSA from 'node-rsa'
import { sha256 } from 'ethers'
import { groth16 } from 'snarkjs'
import {
    Address,
    Hex,
    concat,
    createClient,
    createPublicClient,
    encodeFunctionData,
    http,
    parseEther,
    getContract,
    encodeAbiParameters
} from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { polygonMumbai } from 'viem/chains'
import { UserOperation, bundlerActions, getSenderAddress, getUserOperationHash } from 'permissionless'
import { GetUserOperationGasPriceReturnType, pimlicoBundlerActions } from 'permissionless/actions/pimlico'
import dotenv from 'dotenv'
import { abi as factoryABI } from '../../../packages/contracts/build/ZKMynaWalletFactory.sol/ZKMynaWalletFactory.json'
import { abi as walletABI } from '../../../packages/contracts/build/ZKMynaWallet.sol/ZKMynaWallet.json'
import { abi as paymasterABI } from '../../../packages/contracts/build/MynaWalletPaymaster.sol/MynaWalletPaymaster.json'
import { abi as verifierABI } from '../../../packages/contracts/build/verifier.sol/MynaWalletVerifier.json'
import { PartialBy } from 'viem/_types/types/utils'

// Load environment variables
dotenv.config({ path: join(__dirname, '/../../../.env') })

// Modify the following constants to match your environment in .env file
const PUBLIC_RPC_URL = process.env.PUBLIC_RPC_URL
const BUNDLER_PRC_URL = process.env.BUNDLER_PRC_URL
const ENTRY_POINT_ADDRESS = process.env.ENTRY_POINT_ADDRESS as `0x${string}`
const FACTORY_ADDRESS = process.env.FACTORY_ADDRESS as `0x${string}`
const PAYMASTER_ADDRESS = process.env.PAYMASTER_ADDRESS as `0x${string}`
const GOV_SIG_VERIFIER_ADDRESS = process.env.GOV_SIG_VERIFIER_ADDRESS as `0x${string}`
const USER_SIG_VERIFIER_ADDRESS = process.env.USER_SIG_VERIFIER_ADDRESS as `0x${string}`
// const VERIFIER_ADDRESS = process.env.VERIFIER_ADDRESS as `0x${string}`
const PAYMASTER_SIGNER_PRIVATE_KEY = process.env.PAYMASTER_SIGNER_PRIVATE_KEY as `0x${string}`

const paymasterAccount = privateKeyToAccount(PAYMASTER_SIGNER_PRIVATE_KEY)

// Create clients
const publicClient = createPublicClient({
    chain: polygonMumbai,
    transport: http(PUBLIC_RPC_URL)
})
const pimlicoBundlerClient = createClient({
    chain: polygonMumbai,
    transport: http(BUNDLER_PRC_URL)
})
    .extend(bundlerActions)
    .extend(pimlicoBundlerActions)

async function main() {

    // ----Start: Generate calldata----
    const to = '0x199012076Ea09f92D8C30C494E94738CFF449f57'
    const value = parseEther('0.01')
    const data = '0x'

    const callData = encodeFunctionData({
        abi: walletABI,
        functionName: 'execute',
        args: [to, value, data]
    })
    // ----End: Generate calldata----

    // ----Start: Generate user operation----
    const gasPrice = await getUserOperationGasPrice()

    let userOperation: PartialBy<UserOperation, 'callGasLimit' | 'preVerificationGas' | 'verificationGasLimit'> = {
        sender: senderAddress,
        nonce: isPhantom ? 0n : ((await getNonce(senderAddress)) as bigint),
        initCode: isPhantom ? initCode : '0x',
        callData,
        maxFeePerGas: gasPrice.fast.maxFeePerGas,
        maxPriorityFeePerGas: gasPrice.fast.maxPriorityFeePerGas,
        // dummy signature
        signature: generateDummyZKPSignature(),
        // dummy
        paymasterAndData: '0x'
    }
    // ----End: Generate user operation----

    // ----Start: Sign user operation----
    const userOperationHash = getUserOperationHash({
        userOperation: userOperation as UserOperation,
        entryPoint: ENTRY_POINT_ADDRESS,
        chainId: polygonMumbai.id
    })

    // remove 0x prefix
    const buffer = Buffer.from(userOperationHash.slice(2), 'hex')
    const sha256ed = sha256(buffer)
    console.log('sha256ed hashed:', sha256ed)

    // sign - rsa-sha256 internally
    const key = getTestRsaKey()
    const components = key.exportKey('components')
    const modulus = components.n.toString('hex').replace('00', '')
    const signature = key.sign(buffer, 'buffer', 'buffer')
    console.log('modulus:', modulus)
    console.log('signature:', signature.toString('hex'))

    const m = BigInt(`0x${modulus}`)
    const sign = BigInt(`0x${signature.toString('hex')}`)
    const hashed = BigInt(`${sha256ed}`)

    const INPUT = {
        signature: bigint_to_array(121, 17, sign),
        modulus: bigint_to_array(121, 17, m),
        sha256HashedMessage: bigint_to_array(121, 3, hashed),
        userSecret: '42'
    }
    const { proof, publicSignals } = await groth16.fullProve(
        INPUT,
        join(__dirname, '/../../circom-circuits/build/main_js/main.wasm'),
        join(__dirname, '/../../circom-circuits/setup/circuit_final.zkey')
    )
    const calldata = await groth16.exportSolidityCallData(proof, publicSignals)

    // const isValid = await verifyZKProof(calldata);
    // console.log("Proof isValid:", isValid);

    const argv = calldata
        .replace(/["[\]\s]/g, '')
        .split(',')
        .map(x => x.replace('0x', ''))
        .join('')

    // assign signature
    userOperation.signature = `0x${argv}`
    // ----End: Sign user operation----

    console.log('finalized user operation')
    console.log(userOperation)

    // ----Start: Send user operation to bundler----
    const uoHash = await pimlicoBundlerClient.sendUserOperation({
        userOperation: userOperation as UserOperation,
        entryPoint: ENTRY_POINT_ADDRESS
    })
    console.log('Received User Operation hash:', uoHash)
    // ----End: Send user operation to bundler----

    // let's also wait for the userOperation to be included, by continually querying for the receipts
    console.log('Querying for receipts...')
    const receipt = await pimlicoBundlerClient.waitForUserOperationReceipt({
        hash: uoHash
    })
    const txHash = receipt.receipt.transactionHash

    console.log(`UserOperation included: https://mumbai.polygonscan.com/tx/${txHash}`)
    return
}

main().catch(err => {
    console.error(err)
    process.exit(1)
})

// Helper functions
function createInitCode(identityCommitment: Hex, salt: bigint): Hex {
    const initCode = concat([
        FACTORY_ADDRESS,
        encodeFunctionData({
            abi: factoryABI,
            functionName: 'createAccount',
            args: [identityCommitment, salt]
        })
    ])
    return initCode
}

async function getAddress(initCode: Hex): Promise<[Address, boolean]> {
    const senderAddress = await getSenderAddress(publicClient, {
        initCode,
        entryPoint: ENTRY_POINT_ADDRESS
    })
    const code = await publicClient.getBytecode({ address: senderAddress })
    return [senderAddress, code === undefined]
}

async function getUserOperationGasPrice(): Promise<GetUserOperationGasPriceReturnType> {
    const userOperationGasPrice = await pimlicoBundlerClient.getUserOperationGasPrice()
    return userOperationGasPrice
}

async function getNonce(senderAddress: Address) {
    const account = getContract({
        address: senderAddress,
        abi: walletABI,
        publicClient
    })
    const nonce = await account.read.getNonce()
    return nonce
}

function generateDummyZKPSignature(): Hex {
    return '0x198705050f6c5ec7ce8d0e3beacd67f2143019213cd285db697cb26ce1aff66716c8f21aa09f9dbada9c17d9ec83dd59e484f12c50689a2df48ef41adc9cde6c0e6da20c7b38c80c3f3cc16f8585cca37ddd80d909adcc32a0ae9a524afe195012083fe96e774eedf83bed1d379c40580426c381250d997e7e9dc9222f5e05d504df90947b055418cb10bf5a37eccdb804797dfc9ad53df220bbde205e7ad98006f65c9ff03b25169869a2f65e98f169eb4859c33cbb3f2e86da11572a21d2891f61fa72e55a56acee6cc98bb28be2274ee93e9d0be4d89a4af39ab37e60123b262fcfceeb87403f870fcec0900de2a1c2908b592ec1c98255fe2ecd2fbbf2bf1efdb9c5013a9706e0c16627c36de8dd60731bd96d5bf705bf3db1801267790a00000000000000000000000000000000014a92b6297dc8b5ffb107949f196baf00000000000000000000000000000000009e1187340bc4fd07167462a34b64310000000000000000000000000000000000ed7428f4493d8d819efa6bf71d77c90000000000000000000000000000000000b5d4ef1307ccccefcd331654707e7000000000000000000000000000000000016e2d82bcf9fc45163273840ea8832200000000000000000000000000000000003fb7da2955b7e1c2877da6184af1c800000000000000000000000000000000013876f6867269b091706adf9e6b1ec000000000000000000000000000000000015822b6a7a18159d530974b18a1bb530000000000000000000000000000000001a29c85d823645c5994c167b83090590000000000000000000000000000000001d43619afdf3f353529fdfa1b4f2152000000000000000000000000000000000189975eed5aa142beae1260bf663efd0000000000000000000000000000000001732fd6983070ea2046900a7c3adb430000000000000000000000000000000001564393bb317edbb1cb57cbfa8049ba0000000000000000000000000000000000ac9e95037ab029dae9f9cea8d81c97000000000000000000000000000000000186b47a64af8ea1b79a07d9a97b846b00000000000000000000000000000000007a42797379374010d310f32536bd9f000000000000000000000000000000000000458b290364b9b0bd1d68984c1896000000000000000000000000000000000154b3e7d909e2e281d716c1269b212d0000000000000000000000000000000000482dd4f9954d79b2b46c1e38767ef6000000000000000000000000000000000000000000000000000000000000375c'
}

async function sponsorUserOperation(
    userOperation: PartialBy<UserOperation, 'callGasLimit' | 'preVerificationGas' | 'verificationGasLimit'>
): Promise<UserOperation> {
    // 有効期限終了(0 で指定することで validAfter 以後有効となる)
    const validUntil = 0
    // 有効期限開始
    const validAfter = Math.ceil(Date.now() / 1000)
    // exchangeRate (現状利用しないため 0 )
    const exchangeRate = 0n

    // generate paymasterAndData with dummy signature
    let paymasterAndData = concat([
        PAYMASTER_ADDRESS,
        encodeAbiParameters(
            [
                { name: 'validUntil', type: 'uint48' },
                { name: 'validAfter', type: 'uint48' },
                { name: 'address', type: 'address' },
                { name: 'exchangeRate', type: 'uint256' }
            ],
            [validUntil, validAfter, '0x0000000000000000000000000000000000000000', exchangeRate]
        ),
        // dummy signature
        await paymasterAccount.signMessage({ message: '0xdeadbeef' })
    ])

    // assign dummy paymasterAndData
    userOperation.paymasterAndData = paymasterAndData

    const estimatedGas = await pimlicoBundlerClient.estimateUserOperationGas({
        userOperation,
        entryPoint: ENTRY_POINT_ADDRESS
    })

    // assign estimated gas
    // estimated gas で返される値が信頼できない場合がある
    userOperation.callGasLimit = estimatedGas.callGasLimit
    userOperation.preVerificationGas = estimatedGas.preVerificationGas
    // 4倍しておく
    userOperation.verificationGasLimit =
        estimatedGas.verificationGasLimit * 4n < 500000n ? 500000n : estimatedGas.verificationGasLimit * 4n

    // calculate paymaster hash
    const hash = (await publicClient.readContract({
        address: PAYMASTER_ADDRESS,
        abi: paymasterABI,
        functionName: 'getHash',
        args: [userOperation, validUntil, validAfter, '0x0000000000000000000000000000000000000000', exchangeRate]
    })) as Hex

    const signature = await paymasterAccount.signMessage({
        message: { raw: hash }
    })

    // sign paymaster hash
    userOperation.paymasterAndData = concat([
        PAYMASTER_ADDRESS,
        encodeAbiParameters(
            [
                { name: 'validUntil', type: 'uint48' },
                { name: 'validAfter', type: 'uint48' },
                { name: 'address', type: 'address' },
                { name: 'exchangeRate', type: 'uint256' }
            ],
            [validUntil, validAfter, '0x0000000000000000000000000000000000000000', 0n]
        ),
        signature
    ])

    return userOperation as UserOperation
}

function getTestRsaKey() {
    return new NodeRSA(`-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAj2BHBk9AD9L/gK1lacLP/COAeeLLGGSDBaWbnx84lzD5v5te
PkNviAZcBiQccYm6Q7atvl7HqXnUtC8qRQzRnoB15agXsEMooNFuv8trwJqWAgIX
r2IY83ZdvBKRMe3QBEcqtFkIvwLsNbfAROHJAPffF5/BnJSDWALljEMrxzzuVBSK
byTXMWzKGVeRyH4H6FsH+Atx3cFbmwU+bwJlqOgcJ8dUbeo4y7lRynHDhIkrgd8S
yMsERPngTSTQ0zI/qFcHW+JnRvS3MaGGpRzsJBUVl7nTHJ73jbg/J+8Nlz1NKi2K
kJPHEYv4YyJgOhfXgUoF9hUJY7cqJ19kWgmTGQIDAQABAoIBAHSqkyC/PBGkT+QV
NIBq1XMGMHT95uViZHsj1w4UCah9YbxYYMepeAfnpNoaaEq7F6Yh8B8IYM+3Iy27
c1ncpHWlckn+DciP3W9+++91R6jiIU5hBYTg/gyeNIflU+Cc8reIcWdvS36ikjLj
4sAqObVf/Vjr1k/jST1EniUUQ3tLCKaaePcuHPJ3fcthrKX3doZ9ym2PNn+XYecz
QE6QR5txEowPN7MVyExaP8TaYZdRQGSE0cMruxdbCebhSKgw7PtrIOjq6LPhDzDy
gUG9kLI/lI4FlDkJYC4hOdezycZ+T13KLHYsGnw+dsC4PAqk4w/JVaqgut4jtUU6
pe8XNQECgYEA807085nVLkV6cEJOHRxVtS5oblw8ZI27deg2mQpBBEaD8/H7IpR2
R1nyNj5UV6Ba8UhBGG+9CLjMrm9yuefydPZN6ki+yxK0d1aF20OjllFKEolKJzG7
ZyE+xulCU9cKHajvTJWra9tKvyheNd+ct1Ae5G47DB9pmzKX/uFJclUCgYEAltrd
OrufmMoNnc0N8uha4SM68s3+pz2tGbQ954Qqux3Co+JgQwsVL0OuUT198BG2wSYB
kTySLU3bKHx68Z8p4HMXVzLheJI3eob+2yJ/UIZKjoz3dO4MsFb2KseiWXYeV7Gp
uUE47KVGlfC6gNM32m2iKB361i1YJDo2oxn7ybUCgYEA3toz/Cerng0fP1FL8Nfy
HNhb6LFs04EJ8c32rCg7MupPlBHQv3SR/XqCInLml7gVdCiFDxfRYfq55w/HWkX7
ymuLJArrTl9ckm3afuGuJVFhcibzl4CysJw/vrsJ+HbfGhmQzWnNMCYUiZA08k1V
YoXtNbdNOCZReUhW9apttl0CgYBxiHiVWl2r1O1YhNnppZu38xbLY+MypMVhIfix
BBRQzP4O7zF5Y57m+m338GqWwg4j4WGul8J/3CeDmePBcwNGS/gWBVIRtyGP0od+
DsF4rgjwrgES/JGKKXiNC8AQykfdwfU1WnPoDh9Ie2sxx0Uy2+39eUqt5GSAp1s1
dzm7PQKBgQDTWs2NKZuB4b6o0CEHte+SoINfHvVFDWbotJAZ//l+z1SmTlH7qb+/
jsDhmF/uizOdlopee6fdDaIzYNxEOseI2dx3UjLk6QYqtPBCu9KJ1juSeCReMSjH
BWhALtiQk07pmfH+zFEYEwBhZ0OKaUAZuabat21qFr0cuX1VN8jtBQ==
-----END RSA PRIVATE KEY-----`)
}

function bigint_to_array(n: number, k: number, x: bigint) {
    let mod = 1n
    for (let idx = 0; idx < n; idx++) {
        mod = mod * 2n
    }

    const ret: bigint[] = []
    let x_temp: bigint = x
    for (let idx = 0; idx < k; idx++) {
        ret.push(x_temp % mod)
        x_temp = x_temp / mod
    }
    return ret
}

async function verifyZKProof(callData: string): Promise<boolean> {
    const argv = callData
        .replace(/["[\]\s]/g, '')
        .split(',')
        .map(x => BigInt(x).toString())

    const a = [argv[0], argv[1]]
    const b = [
        [argv[2], argv[3]],
        [argv[4], argv[5]]
    ]
    const c = [argv[6], argv[7]]
    const publicInputs = argv.slice(8)

    try {
        const res = await publicClient.readContract({
            address: VERIFIER_ADDRESS,
            abi: verifierABI,
            functionName: 'verifyProof',
            args: [a, b, c, publicInputs]
        })
        return res as boolean
    } catch (e) {
        console.log(e)
        return false
    }
}
