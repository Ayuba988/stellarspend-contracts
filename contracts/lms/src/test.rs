use crate::{Contract, ContractClient, Error};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

fn setup_course<'a>(
    env: &'a Env,
    lesson_count: u32,
) -> (ContractClient<'a>, Address, Address, Address) {
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    let student = Address::generate(env);
    client.initialize(&admin);
    client.create_course(
        &admin,
        &1,
        &String::from_str(env, "Rust Basics"),
        &lesson_count,
        &25,
        &100,
    );
    (client, admin, student, contract_id)
}

#[test]
fn eligible_student_claims_once_and_event_is_emitted() {
    let env = Env::default();
    let (client, _admin, student, _contract_id) = setup_course(&env, 1);
    client.enroll(&student, &1);
    assert!(client.complete_lesson(&student, &1, &0));

    let events_before_claim = env.events().all().len();
    assert_eq!(client.claim_reward(&student, &1), 25);
    assert_eq!(env.events().all().len(), events_before_claim + 1);
    assert!(matches!(
        client.try_claim_reward(&student, &1),
        Err(Ok(Error::RewardAlreadyClaimed))
    ));
}

#[test]
fn incomplete_course_cannot_be_claimed() {
    let env = Env::default();
    let (client, _admin, student, _contract_id) = setup_course(&env, 2);
    client.enroll(&student, &1);
    client.complete_lesson(&student, &1, &0);

    assert!(matches!(
        client.try_claim_reward(&student, &1),
        Err(Ok(Error::CourseNotCompleted))
    ));
}

#[test]
fn completion_mints_certificate_and_statistics() {
    let env = Env::default();
    env.ledger().set_timestamp(1234);
    let (client, _admin, student, contract_id) = setup_course(&env, 2);
    client.enroll(&student, &1);
    assert!(!client.complete_lesson(&student, &1, &0));
    assert!(client.complete_lesson(&student, &1, &1));

    let stats = client.get_student_statistics(&student);
    assert_eq!(stats.courses_enrolled, 1);
    assert_eq!(stats.courses_completed, 1);
    assert_eq!(stats.lessons_completed, 2);
    assert_eq!(stats.xp, 100);
    assert_eq!(stats.badges, 1);
    assert_eq!(stats.certificates, 1);

    let certificate = client.get_certificate(&1);
    assert!(certificate.exists);
    assert_eq!(certificate.certificate_id, Some(1));
    assert_eq!(certificate.course_id, Some(1));
    assert_eq!(certificate.owner, Some(student));
    assert_eq!(certificate.completion_date, Some(1234));

    let duplicate = env.as_contract(&contract_id, || {
        crate::certificates::mint_certificate(
            &env,
            &student,
            1,
            &String::from_str(&env, "Rust Basics"),
        )
    });
    assert_eq!(duplicate, Err(Error::CertificateAlreadyExists));
}

#[test]
fn empty_statistics_are_zeroed() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let student = Address::generate(&env);

    let stats = client.get_student_statistics(&student);
    assert_eq!(stats.courses_enrolled, 0);
    assert_eq!(stats.courses_completed, 0);
    assert_eq!(stats.lessons_completed, 0);
    assert_eq!(stats.xp, 0);
    assert_eq!(stats.badges, 0);
    assert_eq!(stats.certificates, 0);
}