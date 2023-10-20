use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use chrono::Local;
use ethers_core::abi::{encode, FixedBytes};
use ethers_core::types::*;
use ethers_signers::Signer;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::config::config;
use crate::error::ApiError;
use crate::rpc::RpcRequest;
use crate::types::user_operation::{RpcUserOperation, UserOperation};
use crate::utils::{load_paymaster_contract, load_signer};
use crate::AppState;

const ADDRESS_ZERO: &str = "0x0000000000000000000000000000000000000000";

pub async fn pm_sponsor_user_operation(
    State(state): State<Arc<AppState>>,
    req: RpcRequest,
) -> Result<impl IntoResponse, ApiError> {
    let provider = &state.provider;
    let params = req.params.as_ref().unwrap().as_array().unwrap();
    let (uop, entrypoint) = UserOperation::from(params)?;

    let params = (uop.clone().into(), entrypoint);
    let res = provider
        .request::<(RpcUserOperation, String), EstimateUserOperationGasResult>(
            "eth_estimateUserOperationGas",
            params,
        )
        .await
        .map_err(ApiError::RPCError)
        .unwrap();

    let mut uop = uop.clone();
    uop.call_gas_limit = res.call_gas_limit;
    uop.pre_verification_gas = res.pre_verification_gas;
    uop.verification_gas_limit = res.verification_gas_limit;

    let valid_until = U256::from(0);
    let valid_after = U256::from(Local::now().timestamp());
    let address_zero = ADDRESS_ZERO.parse::<Address>().unwrap();
    let exchange_rate = U256::from(0);

    let contract = load_paymaster_contract(config(), Arc::new(state.provider.clone()));
    let res = contract
        .method::<(UserOperation, U256, U256, Address, U256), FixedBytes>(
            "getHash",
            (
                uop.clone(),
                valid_until,
                valid_after,
                address_zero,
                exchange_rate,
            ),
        )
        .unwrap()
        .call()
        .await
        .map_err(ApiError::ContractError)
        .unwrap();

    let signer = load_signer(config());

    // todo error handling
    let signature = signer.sign_message(&res).await.unwrap();

    let encoded_data = encode(&[
        ethers_core::abi::Token::Uint(valid_until),
        ethers_core::abi::Token::Uint(valid_after),
        ethers_core::abi::Token::Address(address_zero),
        ethers_core::abi::Token::Uint(exchange_rate),
    ]);

    let paymaster_and_data = [
        Bytes::from(contract.address().as_bytes().to_vec()),
        Bytes::from(encoded_data),
        Bytes::from(signature.to_vec()),
    ]
    .into_iter()
    .fold(vec![], |a, b| [a, b.0.to_vec()].concat());

    Ok((
        StatusCode::OK,
        Json(JsonRpcResponse {
            jsonrpc: "2.0",
            id: 1,
            result: PmSponsorUserOperationResponse {
                paymaster_and_data: Bytes::from(paymaster_and_data.to_vec()),
                pre_verification_gas: uop.pre_verification_gas,
                verification_gas: uop.verification_gas_limit,
                verification_gas_limit: uop.verification_gas_limit,
                call_gas_limit: uop.call_gas_limit,
            },
        }),
    )
        .into_response())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct EstimateUserOperationGasResult {
    #[serde(deserialize_with = "integer_to_uint256")]
    pre_verification_gas: U256,
    #[serde(deserialize_with = "integer_to_uint256")]
    verification_gas: U256,
    #[serde(deserialize_with = "integer_to_uint256")]
    verification_gas_limit: U256,
    #[serde(deserialize_with = "integer_to_uint256")]
    call_gas_limit: U256,
}

fn integer_to_uint256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: i128 = Deserialize::deserialize(deserializer)?;
    Ok(U256::from(s))
}

#[derive(Serialize)]
struct JsonRpcResponse<'a> {
    jsonrpc: &'a str,
    id: u8,
    result: PmSponsorUserOperationResponse,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PmSponsorUserOperationResponse {
    paymaster_and_data: Bytes,
    pre_verification_gas: U256,
    verification_gas: U256,
    verification_gas_limit: U256,
    call_gas_limit: U256,
}
