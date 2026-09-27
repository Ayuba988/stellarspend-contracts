//! Shared authentication helpers for StellarSpend contracts.
//!
//! Provides the small set of role checks (`admin` and `owner`) that
//! contracts reuse so authorization logic stays consistent across crates.

use crate::errors::SharedError;
use soroban_sdk::{Address, Env};

/// Requires administrator authentication.
pub fn require_admin(_env: &Env, caller: &Address, admin: &Address) -> Result<(), SharedError> {
    caller.require_auth();
    if caller == admin {
        Ok(())
    } else {
        Err(SharedError::Unauthorized)
    }
}
/// Requires owner authentication.
pub fn require_owner(_env: &Env, caller: &Address, owner: &Address) -> Result<(), SharedError> {
    caller.require_auth();
    if caller == owner {
        Ok(())
    } else {
        Err(SharedError::Unauthorized)
    }
}
