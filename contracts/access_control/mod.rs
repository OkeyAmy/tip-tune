pub mod admin;
pub mod storage;

// Re-export for easy use
pub use admin::{require_admin, transfer_admin};
pub use storage::{set_admin, get_admin};