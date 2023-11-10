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