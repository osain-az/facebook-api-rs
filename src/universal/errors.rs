use std::fmt;

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientErr {
    #[error("facebook error:  {0}")]
    FacebookError(String),
    #[error("facebook custom error:  {0}")]
    CustomError(String),
    #[error("Error from server: {0}")]
    Facebook(#[from] FacebookAPiError),
    #[error("Error from serde: {0}")]
    Serde(#[from] serde_json::error::Error),
    #[error("HTTP client error: {0}")]
    HttpClient(String),
}

#[derive(Deserialize, Debug, Error)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub r#type: String,
    pub error_subcode: Option<u16>,
    pub fbtrace_id: String,
    pub error_user_title: Option<String>,
    pub error_user_msg: Option<String>,
}

#[derive(Deserialize, Debug, Error)]
pub struct FacebookAPiError {
    pub error: ApiError,
}

impl fmt::Display for FacebookAPiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.error)
    }
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}, code: {}", self.message, self.code)
    }
}

impl ApiError {
    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn error_type(&self) -> &str {
        &self.r#type
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn fbtrace_id(&self) -> &str {
        &self.fbtrace_id
    }
}
