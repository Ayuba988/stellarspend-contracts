//! Shared result shapes emitted by batch contracts.

use soroban_sdk::{contracttype, Address, Vec};

/// Shared per-item result shape used by batch contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct BatchItemResult {
    /// Whether this individual operation succeeded.
    pub success: bool,
    /// The address this operation targeted.
    pub target: Address,
    /// The amount processed for this item.
    pub amount: i128,
    /// Contract error code when `success` is `false`; `0` on success.
    pub error_code: u32,
}

/// Shared summary shape for batch execution results.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct BatchExecutionResult {
    /// Total number of items submitted in the batch.
    pub total_requests: u32,
    /// Number of items that succeeded.
    pub successful: u32,
    /// Number of items that failed.
    pub failed: u32,
    /// Per-item results, in submission order.
    pub results: Vec<BatchItemResult>,
}

/// Explicit atomicity policy for a batch contract.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum BatchAtomicityPolicy {
    /// Abort the entire batch if any item is invalid.
    AllOrNothing,
    /// Continue processing the remaining valid items and report failures individually.
    BestEffort,
}
