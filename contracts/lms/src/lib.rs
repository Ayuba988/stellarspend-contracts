#![no_std]

//! Learning management contract for course progress, rewards, analytics, and certificates.

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, String};

pub mod analytics;
pub mod certificates;
pub mod rewards;
mod storage;
pub mod types;

pub use types::{CertificateRecord, Course, StudentStatistics};

/// Errors returned by LMS operations.
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    CourseNotFound = 4,
    InvalidCourse = 5,
    AlreadyEnrolled = 6,
    NotEnrolled = 7,
    InvalidLesson = 8,
    LessonAlreadyCompleted = 9,
    CourseNotCompleted = 10,
    RewardAlreadyClaimed = 11,
    CertificateAlreadyExists = 12,
    IdOverflow = 13,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Initializes the LMS with its course administrator.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::read_config(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        storage::write_config(&env, &types::Config { admin });
        Ok(())
    }

    /// Creates a course and configures its lesson count and completion rewards.
    pub fn create_course(
        env: Env,
        admin: Address,
        course_id: u64,
        title: String,
        lesson_count: u32,
        reward_amount: i128,
        xp_reward: u64,
    ) -> Result<(), Error> {
        admin.require_auth();
        let config = storage::read_config(&env).ok_or(Error::NotInitialized)?;
        if config.admin != admin {
            return Err(Error::Unauthorized);
        }
        if lesson_count == 0 || reward_amount < 0 || storage::read_course(&env, course_id).is_some() {
            return Err(Error::InvalidCourse);
        }
        storage::write_course(
            &env,
            &types::Course {
                course_id,
                title,
                lesson_count,
                reward_amount,
                xp_reward,
            },
        );
        Ok(())
    }

    /// Enrolls an authenticated learner in a course.
    pub fn enroll(env: Env, student: Address, course_id: u64) -> Result<(), Error> {
        student.require_auth();
        storage::read_course(&env, course_id).ok_or(Error::CourseNotFound)?;
        let mut record = storage::read_student(&env, &student).unwrap_or_else(|| {
            types::StudentRecord {
                enrolled_courses: soroban_sdk::Vec::new(&env),
                completed_courses: soroban_sdk::Vec::new(&env),
                xp: 0,
                badges: 0,
            }
        });
        if record.enrolled_courses.iter().any(|id| id == course_id) {
            return Err(Error::AlreadyEnrolled);
        }
        record.enrolled_courses.push_back(course_id);
        storage::write_student(&env, &student, &record);
        storage::write_progress(
            &env,
            &student,
            course_id,
            &types::CourseProgress {
                completed_lessons: soroban_sdk::Vec::new(&env),
                completed: false,
            },
        );
        Ok(())
    }

    /// Records a lesson completion and finalizes the course when all lessons are done.
    pub fn complete_lesson(
        env: Env,
        student: Address,
        course_id: u64,
        lesson_index: u32,
    ) -> Result<bool, Error> {
        student.require_auth();
        let course = storage::read_course(&env, course_id).ok_or(Error::CourseNotFound)?;
        let mut record = storage::read_student(&env, &student).ok_or(Error::NotEnrolled)?;
        if !record.enrolled_courses.iter().any(|id| id == course_id) {
            return Err(Error::NotEnrolled);
        }
        if lesson_index >= course.lesson_count {
            return Err(Error::InvalidLesson);
        }
        let mut progress = storage::read_progress(&env, &student, course_id)
            .ok_or(Error::NotEnrolled)?;
        if progress.completed_lessons.iter().any(|index| index == lesson_index) {
            return Err(Error::LessonAlreadyCompleted);
        }
        progress.completed_lessons.push_back(lesson_index);

        let completed_now = progress.completed_lessons.len() == course.lesson_count;
        if completed_now {
            progress.completed = true;
            record.completed_courses.push_back(course_id);
            record.xp = record.xp.checked_add(course.xp_reward).ok_or(Error::IdOverflow)?;
            record.badges = record.badges.checked_add(1).ok_or(Error::IdOverflow)?;
            certificates::mint_certificate(&env, &student, course_id, &course.title)?;
        }
        storage::write_progress(&env, &student, course_id, &progress);
        storage::write_student(&env, &student, &record);
        Ok(completed_now)
    }

    /// Claims the completion reward for a course, once per learner.
    pub fn claim_reward(env: Env, student: Address, course_id: u64) -> Result<i128, Error> {
        rewards::claim_reward(&env, &student, course_id)
    }

    /// Returns aggregate course and achievement statistics for a learner.
    pub fn get_student_statistics(env: Env, student: Address) -> StudentStatistics {
        analytics::student_statistics(&env, &student)
    }

    /// Returns a certificate by its unique identifier.
    pub fn get_certificate(env: Env, certificate_id: u64) -> certificates::CertificateInfo {
        certificates::get_certificate(&env, certificate_id)
    }
}

#[cfg(test)]
mod test;