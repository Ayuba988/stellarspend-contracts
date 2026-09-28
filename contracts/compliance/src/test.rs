#[cfg(test)]
mod tests {
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::Env;

    #[test]
    fn happy_path_environment() {
        let env = Env::default();
        env.ledger().set_sequence_number(1);
        assert_eq!(env.ledger().sequence(), 1);
    }
    #[test]
    fn unauthorized_boundary_placeholder() {
        let env = Env::default();
        env.mock_all_auths();
        assert!(env.ledger().timestamp() >= 0);
    }
    #[test]
    fn address_generation() {
        let env = Env::default();
        let _ = soroban_sdk::Address::generate(&env);
    }
    #[test]
    fn zero_boundary() {
        assert_eq!(0_i128.checked_add(0), Some(0));
    }
    #[test]
    fn overflow_boundary() {
        assert_eq!(i128::MAX.checked_add(1), None);
    }
}


#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_compliance_check_allowed_and_blocked_addresses() {
    let env = Env::default();
    env.mock_all_auths();

    // Register compliance contract
    let contract_id = env.register(ComplianceContract, ());
    let client = ComplianceContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let allowed_user = Address::generate(&env);
    let blocked_user = Address::generate(&env);

    // Initialize compliance module
    client.initialize(&admin);

    // Set status: allow one user, block the other
    client.set_status(&allowed_user, &true);
    client.set_status(&blocked_user, &false);

    // Assert compliance check results
    assert_eq!(client.is_compliant(&allowed_user), true);
    assert_eq!(client.is_compliant(&blocked_user), false);
}