use soroban_sdk::{contracttype, Address, String, Vec};

/// Course configuration used to validate and reward learner progress.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Course {
    pub course_id: u64,
    pub title: String,
    pub lesson_count: u32,
    pub reward_amount: i128,
    pub xp_reward: u64,
}

/// A learner's progress through one course.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CourseProgress {
    pub completed_lessons: Vec<u32>,
    pub completed: bool,
}

/// Aggregated enrollment and achievement state for one learner.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct StudentRecord {
    pub enrolled_courses: Vec<u64>,
    pub completed_courses: Vec<u64>,
    pub xp: u64,
    pub badges: u64,
}

/// Persisted course-completion certificate.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CertificateRecord {
    pub certificate_id: u64,
    pub course_id: u64,
    pub student: Address,
    pub course: String,
    pub completion_date: u64,
}

/// Learner-facing course and achievement totals.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[contracttype]
pub struct StudentStatistics {
    pub courses_enrolled: u64,
    pub courses_completed: u64,
    pub lessons_completed: u64,
    pub xp: u64,
    pub badges: u64,
    pub certificates: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Course(u64),
    Student(Address),
    Progress(Address, u64),
    RewardClaimed(Address, u64),
    Certificate(u64),
    CertificateFor(Address, u64),
    NextCertificateId,
}

/// Initialized LMS contract administrator.
#[contracttype]
#[derive(Clone)]
pub struct Config {
    pub admin: Address,
}