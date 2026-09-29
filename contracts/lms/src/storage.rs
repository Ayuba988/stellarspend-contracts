use crate::types::{CertificateRecord, Config, Course, CourseProgress, DataKey, StudentRecord};
use soroban_sdk::{Address, Env};

pub fn read_config(env: &Env) -> Option<Config> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn write_config(env: &Env, config: &Config) {
    env.storage().instance().set(&DataKey::Admin, config);
}

pub fn read_course(env: &Env, course_id: u64) -> Option<Course> {
    env.storage().persistent().get(&DataKey::Course(course_id))
}

pub fn write_course(env: &Env, course: &Course) {
    env.storage()
        .persistent()
        .set(&DataKey::Course(course.course_id), course);
}

pub fn read_student(env: &Env, student: &Address) -> Option<StudentRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::Student(student.clone()))
}

pub fn write_student(env: &Env, student: &Address, record: &StudentRecord) {
    env.storage()
        .persistent()
        .set(&DataKey::Student(student.clone()), record);
}

pub fn read_progress(env: &Env, student: &Address, course_id: u64) -> Option<CourseProgress> {
    env.storage()
        .persistent()
        .get(&DataKey::Progress(student.clone(), course_id))
}

pub fn write_progress(env: &Env, student: &Address, course_id: u64, progress: &CourseProgress) {
    env.storage()
        .persistent()
        .set(&DataKey::Progress(student.clone(), course_id), progress);
}

pub fn reward_claimed(env: &Env, student: &Address, course_id: u64) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::RewardClaimed(student.clone(), course_id))
        .unwrap_or(false)
}

pub fn set_reward_claimed(env: &Env, student: &Address, course_id: u64) {
    env.storage()
        .persistent()
        .set(&DataKey::RewardClaimed(student.clone(), course_id), &true);
}

pub fn read_certificate(env: &Env, certificate_id: u64) -> Option<CertificateRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::Certificate(certificate_id))
}

pub fn write_certificate(env: &Env, certificate: &CertificateRecord) {
    env.storage().persistent().set(
        &DataKey::Certificate(certificate.certificate_id),
        certificate,
    );
}

pub fn read_certificate_for(env: &Env, student: &Address, course_id: u64) -> Option<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::CertificateFor(student.clone(), course_id))
}

pub fn write_certificate_for(env: &Env, student: &Address, course_id: u64, id: u64) {
    env.storage()
        .persistent()
        .set(&DataKey::CertificateFor(student.clone(), course_id), &id);
}

pub fn next_certificate_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextCertificateId)
        .unwrap_or(1)
}

pub fn set_next_certificate_id(env: &Env, id: u64) {
    env.storage()
        .instance()
        .set(&DataKey::NextCertificateId, &id);
}