use crate::Error;

/// Validates a budget amount: must be strictly positive.
pub fn validate_amount(amount: i128) -> Result<(), Error> {
    if amount <= 0 {
        Err(Error::InvalidAmount)
    } else {
        Ok(())
    }
}

/// Validates that a budget's end date is strictly after its start date.
pub fn validate_date_range(start_date: u64, end_date: u64) -> Result<(), Error> {
    if end_date <= start_date {
        Err(Error::InvalidDateRange)
    } else {
        Ok(())
    }
}

/// Validates that the proposed budget amount is greater than zero and within permitted limits.
/// 
/// # Arguments
/// * `amount` - A 64-bit unsigned integer representing the budget amount in atomic units.
/// 
/// # Errors
/// Returns `BudgetError::InvalidAmount` if the amount is zero or exceeds maximum bounds.
pub fn validate_amount(amount: u64) -> Result<(), BudgetError> {
    if amount == 0 {
        return Err(BudgetError::InvalidAmount);
    }
    if amount > MAX_BUDGET_LIMIT {
        return Err(BudgetError::ExceedsLimit);
    }
    Ok(())
}

/// Validates that the provided start and end timestamps form a chronologically valid active period.
/// 
/// # Arguments
/// * `start_time` - Unix timestamp marking the beginning of the budget period.
/// * `end_time` - Unix timestamp marking the expiration of the budget period.
/// 
/// # Errors
/// Returns `BudgetError::InvalidTimeRange` if `start_time` is greater than or equal to `end_time`.
pub fn validate_time_range(start_time: u64, end_time: u64) -> Result<(), BudgetError> {
    if start_time >= end_time {
        return Err(BudgetError::InvalidTimeRange);
    }
    Ok(())
}

/// Validates that the caller has sufficient authorization to modify or approve the budget allocation.
/// 
/// # Arguments
/// * `caller` - The Soroban `Address` invoking the budget action.
/// * `authorized_admin` - The designated administrator address for the budget contract.
/// 
/// # Errors
/// Returns `BudgetError::Unauthorized` if the caller does not match the authorized administrator.
pub fn validate_authorization(caller: &Address, authorized_admin: &Address) -> Result<(), BudgetError> {
    caller.require_auth();
    if caller != authorized_admin {
        return Err(BudgetError::Unauthorized);
    }
    Ok(())
}
