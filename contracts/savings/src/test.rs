#[cfg(test)]
mod tests {
    use crate::{Contract, ContractClient, Error};
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

    fn setup(env: &Env) -> ContractClient<'static> {
        env.mock_all_auths();

        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(env, &contract_id);
        let admin = Address::generate(env);

        client.initialize(&admin);

        client
    }

    fn asset(env: &Env, name: &str) -> Symbol {
        Symbol::new(env, name)
    }

    // -------------------------------------------------------------------------
    // Deposit tests
    // -------------------------------------------------------------------------

    #[test]
    fn deposit_increases_balance() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &500_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 500);

        client.deposit(&user, &250_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 750);
    }

    #[test]
    fn deposit_accepts_one_as_smallest_positive_amount() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &1_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 1);
    }

    #[test]
    fn deposit_rejects_zero_amount() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        let result = client.try_deposit(&user, &0_i128, &xlm);

        assert_eq!(result, Err(Ok(Error::InvalidAmount)));
        assert_eq!(client.get_balance(&user, &xlm), 0);
    }

    #[test]
    fn deposit_rejects_negative_amount() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        let result = client.try_deposit(&user, &-10_i128, &xlm);

        assert_eq!(result, Err(Ok(Error::InvalidAmount)));
        assert_eq!(client.get_balance(&user, &xlm), 0);
    }

    #[test]
    fn multiple_deposits_accumulate_correctly() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        for amount in [100_i128, 200, 50, 25] {
            client.deposit(&user, &amount, &xlm);
        }

        assert_eq!(client.get_balance(&user, &xlm), 375);
    }

    // -------------------------------------------------------------------------
    // Withdrawal tests
    // -------------------------------------------------------------------------

    #[test]
    fn withdraw_decreases_balance() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &500_i128, &xlm);
        client.withdraw(&user, &200_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 300);
    }

    #[test]
    fn withdraw_full_balance_leaves_zero() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &500_i128, &xlm);
        client.withdraw(&user, &500_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 0);
    }

    #[test]
    fn withdraw_one_from_balance_leaves_expected_remainder() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &100_i128, &xlm);
        client.withdraw(&user, &1_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 99);
    }

    #[test]
    fn multiple_withdrawals_reduce_balance_correctly() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &1000_i128, &xlm);

        client.withdraw(&user, &100_i128, &xlm);
        client.withdraw(&user, &250_i128, &xlm);
        client.withdraw(&user, &150_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 500);
    }

    #[test]
    fn withdraw_rejects_insufficient_balance() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &100_i128, &xlm);

        let result = client.try_withdraw(&user, &200_i128, &xlm);

        assert_eq!(result, Err(Ok(Error::InsufficientBalance)));

        // Failed withdrawal must not mutate state.
        assert_eq!(client.get_balance(&user, &xlm), 100);
    }

    #[test]
    fn withdraw_rejects_when_no_balance_exists() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        let result = client.try_withdraw(&user, &1_i128, &xlm);

        assert_eq!(result, Err(Ok(Error::InsufficientBalance)));
        assert_eq!(client.get_balance(&user, &xlm), 0);
    }

    #[test]
    fn withdraw_rejects_zero_amount() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &100_i128, &xlm);

        let result = client.try_withdraw(&user, &0_i128, &xlm);

        assert_eq!(result, Err(Ok(Error::InvalidAmount)));
        assert_eq!(client.get_balance(&user, &xlm), 100);
    }

    #[test]
    fn withdraw_rejects_negative_amount() {
        let env = Env::default();
        let client = setup(&env);
        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&user, &100_i128, &xlm);

        let result = client.try_withdraw(&user, &-10_i128, &xlm);

        assert_eq!(result, Err(Ok(Error::InvalidAmount)));
        assert_eq!(client.get_balance(&user, &xlm), 100);
    }

    // -------------------------------------------------------------------------
    // User isolation tests
    // -------------------------------------------------------------------------

    #[test]
    fn balances_are_tracked_per_user() {
        let env = Env::default();
        let client = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&alice, &500_i128, &xlm);
        client.deposit(&bob, &250_i128, &xlm);

        assert_eq!(client.get_balance(&alice, &xlm), 500);
        assert_eq!(client.get_balance(&bob, &xlm), 250);
    }

    #[test]
    fn withdrawing_for_one_user_does_not_affect_another_user() {
        let env = Env::default();
        let client = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&alice, &500_i128, &xlm);
        client.deposit(&bob, &500_i128, &xlm);

        client.withdraw(&alice, &200_i128, &xlm);

        assert_eq!(client.get_balance(&alice, &xlm), 300);
        assert_eq!(client.get_balance(&bob, &xlm), 500);
    }

    // -------------------------------------------------------------------------
    // Asset isolation tests
    // -------------------------------------------------------------------------

    #[test]
    fn balances_are_tracked_per_asset() {
        let env = Env::default();
        let client = setup(&env);

        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");
        let usdc = asset(&env, "USDC");

        client.deposit(&user, &500_i128, &xlm);
        client.deposit(&user, &100_i128, &usdc);

        assert_eq!(client.get_balance(&user, &xlm), 500);
        assert_eq!(client.get_balance(&user, &usdc), 100);
    }

    #[test]
    fn withdrawing_one_asset_does_not_affect_another_asset() {
        let env = Env::default();
        let client = setup(&env);

        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");
        let usdc = asset(&env, "USDC");

        client.deposit(&user, &500_i128, &xlm);
        client.deposit(&user, &100_i128, &usdc);

        client.withdraw(&user, &200_i128, &xlm);

        assert_eq!(client.get_balance(&user, &xlm), 300);
        assert_eq!(client.get_balance(&user, &usdc), 100);
    }

    #[test]
    fn same_asset_balances_are_independent_between_users() {
        let env = Env::default();
        let client = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        client.deposit(&alice, &1000_i128, &xlm);

        assert_eq!(client.get_balance(&alice, &xlm), 1000);
        assert_eq!(client.get_balance(&bob, &xlm), 0);
    }

    // -------------------------------------------------------------------------
    // Balance query tests
    // -------------------------------------------------------------------------

    #[test]
    fn get_balance_defaults_to_zero() {
        let env = Env::default();
        let client = setup(&env);

        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");

        assert_eq!(client.get_balance(&user, &xlm), 0);
    }

    #[test]
    fn unknown_asset_balance_defaults_to_zero() {
        let env = Env::default();
        let client = setup(&env);

        let user = Address::generate(&env);
        let xlm = asset(&env, "XLM");
        let usdc = asset(&env, "USDC");

        client.deposit(&user, &500_i128, &xlm);

        assert_eq!(client.get_balance(&user, &usdc), 0);
    }

    // -------------------------------------------------------------------------
    // Initialization tests
    // -------------------------------------------------------------------------

    #[test]
    fn initialize_succeeds_once() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);

        client.initialize(&admin);
    }

    #[test]
    fn double_initialize_fails() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);

        client.initialize(&admin);

        let result = client.try_initialize(&admin);

        assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
    }

    #[test]
    fn second_initialize_does_not_replace_existing_admin() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let first_admin = Address::generate(&env);
        let second_admin = Address::generate(&env);

        client.initialize(&first_admin);

        let result = client.try_initialize(&second_admin);

        assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
    }
}