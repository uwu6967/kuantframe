use qf_api::errors::ApiError as QFRequestError;
use serde_json::json;
use utils::{Error, LogLevel, Properties};
use wf_market::errors::ApiError as WFRequestError;

/// Variant name for a Warframe Market API error.
///
/// The published `wf-market` crate does not expose this. Upstream development
/// calls it while tracking events, so the label lives here.
pub trait WfmApiErrorExt {
    fn error_type(&self) -> &'static str;
}

impl WfmApiErrorExt for WFRequestError {
    fn error_type(&self) -> &'static str {
        match self {
            WFRequestError::TooManyRequests(_) => "TooManyRequests",
            WFRequestError::RequestError(_) => "RequestError",
            WFRequestError::Unauthorized(_) => "Unauthorized",
            WFRequestError::ParsingError(_, _) => "ParsingError",
            WFRequestError::NotFound(_) => "NotFound",
            WFRequestError::BadRequest(_) => "BadRequest",
            WFRequestError::InvalidCredentials(_) => "InvalidCredentials",
            WFRequestError::Forbidden(_) => "Forbidden",
            WFRequestError::EndOfFile(_) => "EndOfFile",
            WFRequestError::InternalServerError(_) => "InternalServerError",
            WFRequestError::OrderLimitExceeded(_) => "OrderLimitExceeded",
            WFRequestError::OrderLimitExceededSamePrice(_) => "OrderLimitExceededSamePrice",
            WFRequestError::AuctionLimitExceeded(_) => "AuctionLimitExceeded",
            WFRequestError::InvalidType { .. } => "InvalidType",
            WFRequestError::Unknown(_) => "Unknown",
        }
    }
}

use crate::SENSITIVE_FIELDS;

/// Extension trait for creating Error instances from different error types
pub trait ErrorFromExt {
    /// Create an Error from a Warframe Market API error
    fn from_wfm(
        component: impl Into<String>,
        message: impl Into<String>,
        error: WFRequestError,
        location: impl Into<String>,
    ) -> Self;

    /// Create an Error from a QuantFrame API error
    fn from_qf(
        component: impl Into<String>,
        message: impl Into<String>,
        error: QFRequestError,
        location: impl Into<String>,
    ) -> Self;
    fn new_permission_denied(flag: impl Into<String>) -> Self;
}

impl ErrorFromExt for Error {
    fn from_wfm(
        component: impl Into<String>,
        message: impl Into<String>,
        mut error: WFRequestError,
        location: impl Into<String>,
    ) -> Self {
        error.mask_sensitive_data(SENSITIVE_FIELDS);
        Error {
            component: format!("WFMClient:{}", component.into()),
            cause: error.to_string(),
            message: message.into(),
            log_level: LogLevel::Critical,
            properties: Properties::from(error.to_json()),
            location: Some(location.into()),
        }
    }

    fn from_qf(
        component: impl Into<String>,
        message: impl Into<String>,
        mut error: QFRequestError,
        location: impl Into<String>,
    ) -> Self {
        error.mask_sensitive_data(SENSITIVE_FIELDS);
        Error {
            component: format!("QFClient:{}", component.into()),
            cause: error.to_string(),
            message: message.into(),
            log_level: LogLevel::Critical,
            properties: Properties::from(json!(error.to_string())),
            location: Some(location.into()),
        }
    }
    fn new_permission_denied(flag: impl Into<String>) -> Self {
        let flag = flag.into();
        Error {
            component: "AuthModule".into(),
            cause: "Permission Denied".to_string(),
            message: format!("User does not have permission: {}", flag),
            log_level: LogLevel::Warning,
            properties: Properties::from(json!({ "flag": flag })),
            location: None,
        }
    }
}
