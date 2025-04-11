//! Constants that are used in the domain level.

/// API version
#[cfg(feature = "v1")]
pub const API_VERSION: common_enums::ApiVersion = common_enums::ApiVersion::V1;

/// API version
#[cfg(feature = "v2")]
pub const API_VERSION: common_enums::ApiVersion = common_enums::ApiVersion::V2;

/// No error message string const
pub const NO_ERROR_MESSAGE: &str = "No error message";

/// No error code string const
pub const NO_ERROR_CODE: &str = "No error code";
