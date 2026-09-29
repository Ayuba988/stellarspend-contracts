#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Env,
    };

    #[test]
    fn ledger_sequence_can_be_updated() {
        let env = Env::default();

        assert_eq!(env.ledger().sequence(), 0);

        env.ledger().set_sequence_number(1);

        assert_eq!(env.ledger().sequence(), 1);
    }

    #[test]
    fn mocked_authentication_can_be_enabled() {
        let env = Env::default();

        env.mock_all_auths();

        // The environment should remain functional after enabling
        // mocked authentication.
        env.ledger().set_sequence_number(42);

        assert_eq!(env.ledger().sequence(), 42);
    }

    #[test]
    fn generated_addresses_are_unique() {
        let env = Env::default();

        let address_a = Address::generate(&env);
        let address_b = Address::generate(&env);

        assert_ne!(address_a, address_b);
    }

    #[test]
    fn zero_arithmetic_is_safe() {
        assert_eq!(0_i128.checked_add(0), Some(0));
        assert_eq!(0_i128.checked_sub(0), Some(0));
        assert_eq!(0_i128.checked_mul(0), Some(0));
    }

    #[test]
    fn positive_integer_overflow_is_detected() {
        assert_eq!(i128::MAX.checked_add(1), None);
        assert_eq!(i128::MAX.checked_mul(2), None);
    }

    #[test]
    fn negative_integer_overflow_is_detected() {
        assert_eq!(i128::MIN.checked_sub(1), None);
        assert_eq!(i128::MIN.checked_mul(2), None);
    }

    #[test]
    fn integer_boundaries_are_preserved() {
        assert_eq!(i128::MAX.checked_add(0), Some(i128::MAX));
        assert_eq!(i128::MIN.checked_sub(0), Some(i128::MIN));
    }
}
