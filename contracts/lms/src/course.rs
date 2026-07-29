use soroban_sdk::{contracterror, contracttype, symbol_short, Address, Env, Symbol};
use crate::storage::StorageKey;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CourseError {
    CourseNotFound = 1,
    AlreadyPublished = 2,
    NotAuthorized = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub id: u64,
    pub admin: Address,
    pub title: soroban_sdk::String,
    pub published: bool,
}

pub fn publish_course(env: Env, caller: Address, course_id: u64) -> Result<(), CourseError> {
    // Require signature from caller
    caller.require_auth();

    let key = StorageKey::Course(course_id);

    // 1. Prevent publishing non-existent courses
    let mut course: Course = env
        .storage()
        .instance()
        .get(&key)
        .ok_or(CourseError::CourseNotFound)?;

    // 2. Ensure only authorized course admin can publish
    if course.admin != caller {
        return Err(CourseError::NotAuthorized);
    }

    // Optional: Return error if already published
    if course.published {
        return Err(CourseError::AlreadyPublished);
    }

    // 3. Toggle published status
    course.published = true;
    env.storage().instance().set(&key, &course);

    // 4. Emit CoursePublished event
    // Topics: ("course_published", course_id), Data: caller
    env.events().publish(
        (symbol_short!("published"), course_id),
        caller,
    );

    Ok(())
}