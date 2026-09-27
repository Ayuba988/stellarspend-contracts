//! Shared input-validation helpers for StellarSpend contracts.

use crate::errors::SharedError;
use soroban_sdk::{Address, Env};

/// Validates that a financial `amount` is strictly positive.
///
/// Returns [`SharedError::InvalidAmount`] when `amount` is zero or negative.
pub fn validate_positive_amount(amount: i128) -> Result<(), SharedError> {
    if amount > 0 {
        Ok(())
    } else {
        Err(SharedError::InvalidAmount)
    }
}
/// Validates a Soroban address.
///
/// Always returns `Ok(())`: the SDK has already type-checked the address, so
/// there is nothing further to reject here.
pub fn validate_address(_env: &Env, _addr: &Address) -> Result<(), SharedError> {
    Ok(())
}
/// Validates that a string's length falls within the inclusive `min..=max`
/// bounds.
///
/// Returns [`SharedError::InvalidString`] when the length is outside that
/// range.
pub fn validate_string_length(s: &str, min: u32, max: u32) -> Result<(), SharedError> {
    let n = s.len() as u32;
    if n < min || n > max {
        Err(SharedError::InvalidString)
    } else {
        Ok(())
    }
}
