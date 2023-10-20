use axum::{http::StatusCode, response::IntoResponse, Json};
use ethers::contract::ContractError;
use ethers_providers::{Http, Provider, ProviderError};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    ConfigParseError(&'static str),
}

#[derive(Debug)]
pub enum ApiError {
    InvalidMethodError(String),
    JSONExtractError(String),
    InvalidJSON(ValidationErrors),
    InvalidJSONParametersError(String),
    RPCError(ProviderError),
    ContractError(ContractError<Provider<Http>>),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub jsonrpc: String,
    pub id: u8,
    error: ErrorBody,
}

#[derive(Serialize, Deserialize)]
struct ErrorBody {
    code: i16,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (error_code, message) = match self {
            ApiError::InvalidMethodError(err) => (-32601, err),
            ApiError::InvalidJSONParametersError(err) => (-32602, err),
            ApiError::JSONExtractError(err) => (-32603, err),
            ApiError::InvalidJSON(err) => (-32604, format!("{:?}", err)),
            ApiError::RPCError(err) => (-32605, format!("{:?}", err)),
            ApiError::ContractError(err) => (-32606, format!("{:?}", err)),
        };

        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                jsonrpc: "2.0".to_string(),
                id: 1,
                error: ErrorBody {
                    code: error_code,
                    message,
                },
            }),
        )
            .into_response()
    }
}

impl From<ApiError> for axum::response::Response {
    fn from(error: ApiError) -> Self {
        error.into_response()
    }
}
