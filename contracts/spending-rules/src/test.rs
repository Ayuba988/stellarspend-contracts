#![cfg(test)]

use crate::{RuleDecision, SpendingRulesContract, SpendingRulesContractClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Ledger},
    Address, Bytes, Env, Symbol, Vec,
};
use spending_categories::{SpendingCategoriesContract, SpendingCategoriesContractClient};
use spending_limits::{
    LimitStrategy, SpendingLimitRequest, SpendingLimitsContract, SpendingLimitsContractClient,
};
use zk_verifier::ZkVerifierContract;

const XLM: i128 = 10_000_000;
const WEEK: u64 = 7 * 86_400;

struct TestHarness {
    env: Env,
    admin: Address,
    user: Address,
    category: Symbol,
    rules: SpendingRulesContractClient<'static>,
    limits: SpendingLimitsContractClient<'static>,
}

fn setup() -> TestHarness {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(86_400);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let category = symbol_short!("Groceries");

    let categories_id = env.register(SpendingCategoriesContract, ());
    let categories = SpendingCategoriesContractClient::new(&env, &categories_id);
    categories.initialize(&admin);
    categories.create_category(&admin, &user, &category);

    let verifier_id = env.register(ZkVerifierContract, ());
    let limits_id = env.register(SpendingLimitsContract, ());
    let limits = SpendingLimitsContractClient::new(&env, &limits_id);
    limits.initialize(&admin);
    limits.whitelist_destination(&admin, &user);

    let mut requests = Vec::new(&env);
    requests.push_back(SpendingLimitRequest {
        user: user.clone(),
        monthly_limit: 10_000 * XLM,
        daily_limit: 10_000 * XLM,
        hourly_limit: 10_000 * XLM,
        reset_window_seconds: 86_400,
        category: Some(category.clone()),
        strategy: LimitStrategy::Static,
    });
    limits.batch_update_spending_limits(&admin, &requests);

    let rules_id = env.register(SpendingRulesContract, ());
    let rules = SpendingRulesContractClient::new(&env, &rules_id);
    rules.initialize(&admin, &categories_id, &verifier_id);

    TestHarness {
        env,
        admin,
        user,
        category,
        rules,
        limits,
    }
}

fn valid_proof(env: &Env) -> Bytes {
    let mut proof = Bytes::new(env);
    proof.push_back(1);
    proof
}

#[test]
fn payment_under_all_thresholds_succeeds_without_proof() {
    let h = setup();
    h.rules.add_rule(
        &h.admin,
        &h.user,
        &Some(h.category.clone()),
        &(200 * XLM),
        &WEEK,
        &Some(h.limits.address.clone()),
        &Some(100 * XLM),
    );

    let result = h
        .rules
        .enforce_payment(&h.user, &(50 * XLM), &Some(h.category.clone()), &None);

    assert!(result.allowed);
    assert_eq!(result.decision, RuleDecision::Allow);
    assert!(!result.requires_zk);
    assert_eq!(h.rules.get_rule_usage(&1), 50 * XLM);
}

#[test]
fn payment_above_zk_threshold_fails_without_proof_and_succeeds_with_valid_proof() {
    let h = setup();
    h.rules.add_rule(
        &h.admin,
        &h.user,
        &Some(h.category.clone()),
        &(200 * XLM),
        &WEEK,
        &Some(h.limits.address.clone()),
        &Some(100 * XLM),
    );

    let without_proof =
        h.rules
            .evaluate_payment(&h.user, &(150 * XLM), &Some(h.category.clone()), &None);
    assert!(!without_proof.allowed);
    assert_eq!(without_proof.decision, RuleDecision::RequireZkProof);
    assert_eq!(h.rules.get_rule_usage(&1), 0);

    let proof = valid_proof(&h.env);
    let with_proof = h.rules.enforce_payment(
        &h.user,
        &(150 * XLM),
        &Some(h.category.clone()),
        &Some(proof),
    );
    assert!(with_proof.allowed);
    assert!(with_proof.requires_zk);
    assert_eq!(h.rules.get_rule_usage(&1), 150 * XLM);
}

#[test]
fn category_cap_and_zk_gate_are_both_enforced() {
    let h = setup();
    h.rules.add_rule(
        &h.admin,
        &h.user,
        &Some(h.category.clone()),
        &(200 * XLM),
        &WEEK,
        &Some(h.limits.address.clone()),
        &None,
    );
    h.rules.add_rule(
        &h.admin,
        &h.user,
        &None::<Symbol>,
        &(10_000 * XLM),
        &WEEK,
        &None::<Address>,
        &Some(100 * XLM),
    );

    let no_proof =
        h.rules
            .evaluate_payment(&h.user, &(150 * XLM), &Some(h.category.clone()), &None);
    assert!(!no_proof.allowed);
    assert_eq!(no_proof.decision, RuleDecision::RequireZkProof);

    let proof = valid_proof(&h.env);
    let approved = h.rules.enforce_payment(
        &h.user,
        &(150 * XLM),
        &Some(h.category.clone()),
        &Some(proof),
    );
    assert!(approved.allowed);

    let proof = valid_proof(&h.env);
    let over_weekly_cap = h.rules.evaluate_payment(
        &h.user,
        &(60 * XLM),
        &Some(h.category.clone()),
        &Some(proof),
    );
    assert!(!over_weekly_cap.allowed);
    assert_eq!(over_weekly_cap.decision, RuleDecision::Deny);
    assert_eq!(over_weekly_cap.blocking_rule, Some(1));
}

#[test]
fn most_restrictive_rule_wins_when_caps_conflict() {
    let h = setup();
    h.rules.add_rule(
        &h.admin,
        &h.user,
        &Some(h.category.clone()),
        &(200 * XLM),
        &WEEK,
        &None::<Address>,
        &None,
    );
    h.rules.add_rule(
        &h.admin,
        &h.user,
        &Some(h.category.clone()),
        &(125 * XLM),
        &WEEK,
        &None::<Address>,
        &None,
    );

    let result = h
        .rules
        .evaluate_payment(&h.user, &(150 * XLM), &Some(h.category.clone()), &None);

    assert!(!result.allowed);
    assert_eq!(result.decision, RuleDecision::Deny);
    assert_eq!(result.blocking_rule, Some(2));
}
