//! Eligibility checks and one-time claims for course completion rewards.

use crate::{storage, Error};
use soroban_sdk::{Address, Env, Symbol};

/// Claims the configured reward for a course the learner has completed.
pub fn claim_reward(env: &Env, student: &Address, course_id: u64) -> Result<i128, Error> {
    student.require_auth();
    let course = storage::read_course(env, course_id).ok_or(Error::CourseNotFound)?;
    let progress = storage::read_progress(env, student, course_id)
        .ok_or(Error::CourseNotCompleted)?;
    if !progress.completed {
        return Err(Error::CourseNotCompleted);
    }
    if storage::reward_claimed(env, student, course_id) {
        return Err(Error::RewardAlreadyClaimed);
    }

    storage::set_reward_claimed(env, student, course_id);
    env.events().publish(
        (Symbol::new(env, "RewardClaimed"), course_id),
        (student.clone(), course.reward_amount),
    );
    Ok(course.reward_amount)
}

/// Returns whether the learner has already claimed this course's reward.
pub fn has_claimed(env: &Env, student: &Address, course_id: u64) -> bool {
    storage::reward_claimed(env, student, course_id)
}