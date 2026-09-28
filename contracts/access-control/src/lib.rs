#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

mod storage;
#[cfg(test)]
mod test;
pub mod types;
pub mod validation;

/// Errors returned by the access-control contract.
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The contract has already been initialized.
    AlreadyInitialized = 1,

    /// The caller is not the configured administrator.
    Unauthorized = 2,

    /// The supplied value is invalid.
    InvalidAmount = 3,
}

/// Access-control contract.
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Initializes the contract with the specified administrator.
    ///
    /// Initialization can only be performed once and requires authorization
    /// from the administrator address being configured.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::read_config(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();

        storage::write_config(
            &env,
            &types::Config {
                admin,
                value: 0,
            },
        );

        Ok(())
    }

    /// Updates the contract value.
    ///
    /// Only the configured administrator can update the value, and the value
    /// must be greater than or equal to zero.
    pub fn set_value(env: Env, admin: Address, value: i128) -> Result<(), Error> {
        let config = storage::read_config(&env).ok_or(Error::Unauthorized)?;

        if config.admin != admin {
            return Err(Error::Unauthorized);
        }

        admin.require_auth();

        if value < 0 {
            return Err(Error::InvalidAmount);
        }

        storage::write_config(
            &env,
            &types::Config {
                admin,
                value,
            },
        );

        Ok(())
    }

    /// Returns the currently configured contract value.
    ///
    /// Returns `0` if the contract has not yet been initialized.
    pub fn get_value(env: Env) -> i128 {
        match storage::read_config(&env) {
            Some(config) => config.value,
            None => 0,
        }
    }
}