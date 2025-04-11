//! connector integration related const declarations

pub use common_types::consts::{NO_ERROR_CODE, NO_ERROR_MESSAGE};

/// Accepted format for request
pub const ACCEPT_HEADER: &str = "text/html,application/json";

/// User agent for request send from backend server
pub const USER_AGENT: &str = "Hyperswitch-Backend-Server";
