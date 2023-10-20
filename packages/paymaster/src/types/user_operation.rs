use ethers_core::abi::{Token, Tokenizable};
use ethers_core::types::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::error::ApiError;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

impl UserOperation {
    pub fn from(params: &[Value]) -> Result<(UserOperation, String), ApiError> {
        let mut user_operation: Option<UserOperation> = None;
        let mut entry_point_addr = String::new();

        for param in params.iter() {
            if param.is_object() {
                match from_value::<UserOperation>(param.clone()) {
                    Ok(op) => user_operation = Some(op),
                    Err(err) => {
                        return Err(ApiError::InvalidJSONParametersError(
                            format!("Failed to deserialize UserOperation. {:?}", err).to_string(),
                        ));
                    }
                }
            } else if param.is_string() {
                entry_point_addr = param.as_str().unwrap().to_string();
            } else {
                return Err(ApiError::InvalidJSONParametersError(
                    "Invalid params for pm_sponsorUserOperation.".to_string(),
                ));
            }
        }

        if user_operation.is_none() {
            return Err(ApiError::InvalidJSONParametersError(
                "Invalid params: UserOperation Not Found".to_string(),
            ));
        }
        if entry_point_addr.is_empty() {
            return Err(ApiError::InvalidJSONParametersError(
                "Invalid params: EntryPoint Not Found".to_string(),
            ));
        }

        Ok((user_operation.unwrap(), entry_point_addr))
    }
}

impl Tokenizable for UserOperation {
    fn into_token(self) -> Token {
        Token::Tuple(vec![
            Token::Address(self.sender),
            Token::Uint(self.nonce),
            Token::Bytes(self.init_code.to_vec()),
            Token::Bytes(self.call_data.to_vec()),
            Token::Uint(self.call_gas_limit),
            Token::Uint(self.verification_gas_limit),
            Token::Uint(self.pre_verification_gas),
            Token::Uint(self.max_fee_per_gas),
            Token::Uint(self.max_priority_fee_per_gas),
            Token::Bytes(self.paymaster_and_data.to_vec()),
            Token::Bytes(self.signature.to_vec()),
        ])
    }

    fn from_token(token: Token) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcUserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: String,
    pub verification_gas_limit: String,
    pub pre_verification_gas: String,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

impl From<UserOperation> for RpcUserOperation {
    fn from(value: UserOperation) -> Self {
        Self {
            sender: value.sender,
            nonce: value.nonce,
            call_gas_limit: format!("0x{:x}", value.call_gas_limit),
            init_code: value.init_code,
            call_data: value.call_data,
            verification_gas_limit: format!("0x{:x}", value.verification_gas_limit),
            pre_verification_gas: format!("0x{:x}", value.pre_verification_gas),
            max_fee_per_gas: value.max_fee_per_gas,
            max_priority_fee_per_gas: value.max_priority_fee_per_gas,
            paymaster_and_data: value.paymaster_and_data,
            signature: value.signature,
        }
    }
}
