use soroban_sdk::{contracttype, Address, Symbol};

/// Global configuration for the contract.
///
/// The administrator is stored as part of the contract configuration so that
/// future administrative functionality can be added without changing the
/// configuration structure.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    /// Address of the contract administrator.
    pub admin: Address,
}

/// Persistent storage keys used by the contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Global contract configuration.
    Config,

    /// Balance associated with a specific user and asset.
    ///
    /// Each `(Address, Symbol)` pair maintains an independent balance.
    Balance(Address, Symbol),
}