//! Certificate issuance and authenticity verification for completed courses.

use crate::{storage, types::CertificateRecord, Error};
use soroban_sdk::{contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub struct CertificateInfo {
    pub exists: bool,
    pub certificate_id: Option<u64>,
    pub course_id: Option<u64>,
    pub owner: Option<Address>,
    pub course: Option<String>,
    pub completion_date: Option<u64>,
}

/// Creates one certificate for a learner's completed course.
pub fn mint_certificate(
    env: &Env,
    student: &Address,
    course_id: u64,
    course: &String,
) -> Result<u64, Error> {
    if storage::read_certificate_for(env, student, course_id).is_some() {
        return Err(Error::CertificateAlreadyExists);
    }

    let certificate_id = storage::next_certificate_id(env);
    let certificate = CertificateRecord {
        certificate_id,
        course_id,
        student: student.clone(),
        course: course.clone(),
        completion_date: env.ledger().timestamp(),
    };
    storage::write_certificate(env, &certificate);
    storage::write_certificate_for(env, student, course_id, certificate_id);
    storage::set_next_certificate_id(
        env,
        certificate_id.checked_add(1).ok_or(Error::IdOverflow)?,
    );
    env.events().publish(
        (soroban_sdk::symbol_short!("cert_mint"), certificate_id),
        (student.clone(), course_id),
    );
    Ok(certificate_id)
}

/// Returns the stored certificate for an identifier, if it exists.
pub fn get_certificate(env: &Env, certificate_id: u64) -> CertificateInfo {
    match storage::read_certificate(env, certificate_id) {
        Some(certificate) => CertificateInfo {
            exists: true,
            certificate_id: Some(certificate.certificate_id),
            course_id: Some(certificate.course_id),
            owner: Some(certificate.student),
            course: Some(certificate.course),
            completion_date: Some(certificate.completion_date),
        },
        None => CertificateInfo {
            exists: false,
            certificate_id: None,
            course_id: None,
            owner: None,
            course: None,
            completion_date: None,
        },
    }
}

/// Verifies a certificate by id and returns its existence, owner,
/// course, and completion date. `lookup` is the storage read for the
/// certificate record, injected so this stays testable without wiring
/// full contract storage yet.
pub fn verify_certificate(
    _env: &Env,
    certificate_id: u64,
    lookup: impl Fn(u64) -> Option<(Address, String, u64)>,
) -> CertificateInfo {
    match lookup(certificate_id) {
        Some((owner, course, completion_date)) => CertificateInfo {
            exists: true,
            certificate_id: Some(certificate_id),
            course_id: None,
            owner: Some(owner),
            course: Some(course),
            completion_date: Some(completion_date),
        },
        None => CertificateInfo {
            exists: false,
            certificate_id: None,
            course_id: None,
            owner: None,
            course: None,
            completion_date: None,
        },
    }
}
