use soroban_sdk::{Env, Address};
use super::storage::get_admin;

/// 🔒 Require admin authentication
pub fn require_admin(env: &Env) -> Address {
    let admin = get_admin(env);

    // Enforce authentication
    admin.require_auth();

    admin
}

/// 🔄 Optional: transfer admin ownership
pub fn transfer_admin(env: &Env, new_admin: Address) {
    let current_admin = require_admin(env);

    // Prevent accidental self-transfer (optional safety)
    if current_admin == new_admin {
        panic!("New admin must be different");
    }

    super::storage::set_admin(env, &new_admin);
}