use serde::Serialize;
use crate::enums::errors::ModelError;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub type ModelResponse<T> = Result<T, ModelError>;