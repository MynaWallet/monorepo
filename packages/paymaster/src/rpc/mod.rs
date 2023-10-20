mod pm_sponsor_user_operation;

use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, State},
    response::IntoResponse,
    Json, RequestExt,
};
use hyper::Request;
use serde::Deserialize;
use serde_json::Value;
use validator::{Validate, ValidationError};

use crate::error::ApiError;
use crate::AppState;
use pm_sponsor_user_operation::pm_sponsor_user_operation;

pub async fn handle_rpc_request(
    state: State<Arc<AppState>>,
    req: ValidatedJson<RpcRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let method_name = req.0.method.as_deref().unwrap_or_default();
    match method_name {
        "pm_sponsorUserOperation" => Ok(pm_sponsor_user_operation(state, req.0).await?),
        _ => Err(ApiError::InvalidMethodError("Invalid method".to_string())),
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RpcRequest {
    #[validate(required)]
    pub jsonrpc: Option<String>,
    #[validate(required, range(min = 1, max = 1))]
    pub id: Option<u8>,
    #[validate(required)]
    pub method: Option<String>,
    #[validate(required, custom = "validate_params")]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<J>(pub J);

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedJson<J>
where
    B: Send + 'static,
    S: Send + Sync + 'static,
    J: Validate + 'static,
    Json<J>: FromRequest<(), B>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req.extract::<Json<J>, _>().await.map_err(|_| {
            ApiError::JSONExtractError("Failed to extract request body".to_string())
        })?;

        data.validate().map_err(ApiError::InvalidJSON)?;
        Ok(Self(data))
    }
}

fn validate_params(params: &Value) -> Result<(), ValidationError> {
    if !params.is_array() {
        return Err(ValidationError::new("Invalid params"));
    }
    Ok(())
}
