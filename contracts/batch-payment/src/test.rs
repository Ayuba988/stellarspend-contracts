
#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Env,
    };

    #[test]
    fn should_set_and_read_ledger_sequence() {
        let env = Env::default();

        env.ledger().set_sequence_number(1);

        assert_eq!(env.ledger().sequence(), 1);
    }

    #[test]
    fn should_support_mocked_authentication() {
        let env = Env::default();

        env.mock_all_auths();

        // Authentication is mocked for subsequent contract calls.
        // This test mainly verifies that the test environment accepts
        // the mocked-auth configuration.
        assert_eq!(env.ledger().sequence(), 0);
    }

    #[test]
    fn should_generate_unique_addresses() {
        let env = Env::default();

        let first = Address::generate(&env);
        let second = Address::generate(&env);

        assert_ne!(first, second);
    }

    #[test]
    fn should_handle_zero_boundary() {
        assert_eq!(0_i128.checked_add(0), Some(0));
        assert_eq!(0_i128.checked_sub(0), Some(0));
    }

    #[test]
    fn should_detect_integer_overflow() {
        assert_eq!(i128::MAX.checked_add(1), None);
        assert_eq!(i128::MIN.checked_sub(1), None);
    }
}
