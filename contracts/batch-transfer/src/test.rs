
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
    fn should_enable_mock_authentication() {
        let env = Env::default();

        env.mock_all_auths();

        // Verify that the environment remains usable after enabling
        // mocked authentication.
        assert_eq!(env.ledger().sequence(), 0);
    }

    #[test]
    fn should_generate_distinct_addresses() {
        let env = Env::default();

        let first_address = Address::generate(&env);
        let second_address = Address::generate(&env);

        assert_ne!(first_address, second_address);
    }

    #[test]
    fn should_handle_zero_values() {
        assert_eq!(0_i128.checked_add(0), Some(0));
        assert_eq!(0_i128.checked_sub(0), Some(0));
        assert_eq!(0_i128.checked_mul(0), Some(0));
    }

    #[test]
    fn should_detect_positive_overflow() {
        assert_eq!(i128::MAX.checked_add(1), None);
        assert_eq!(i128::MAX.checked_mul(2), None);
    }

    #[test]
    fn should_detect_negative_overflow() {
        assert_eq!(i128::MIN.checked_sub(1), None);
        assert_eq!(i128::MIN.checked_mul(2), None);
    }
}

