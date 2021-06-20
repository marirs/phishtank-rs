use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PhishtankError {
    #[error("You must supply a URL to use this function.")]
    InvalidUrl,
    #[error("The server has exceeded the bandwidth specified by the server administrator.")]
    BandwidthExceeded,
    #[error("Unknown error.")]
    Unknown,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(serde_json::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

impl From<serde_json::Error> for PhishtankError {
    fn from(err: serde_json::Error) -> PhishtankError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => PhishtankError::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => PhishtankError::Json(err),
        }
    }
}

/// Return the PhishtankError based on the http status code
impl From<StatusCode> for PhishtankError {
    fn from(s: StatusCode) -> PhishtankError {
        match s.as_str() {
            "509" => PhishtankError::BandwidthExceeded,
            _ => PhishtankError::Unknown,
        }
    }
}
