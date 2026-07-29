use soroban_sdk::{contracterror, contracttype, Address, Env, String};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CourseError {
    CourseNotFound = 1,
    Unauthorized = 2,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Course {
    pub id: u64,
    pub instructor: Address,
    pub title: String,
    pub description: String,
    pub category: String,
    pub difficulty: u32,
    pub thumbnail: String,
    pub published: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct UpdateCourseInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub difficulty: Option<u32>,
    pub thumbnail: Option<String>,
    pub published: Option<bool>,
}

#[contracttype]
pub enum DataKey {
    Course(u64),
}

pub fn update_course(
    env: Env,
    caller: Address,
    course_id: u64,
    input: UpdateCourseInput,
) -> Result<Course, CourseError> {
    // 1. Verify caller signature
    caller.require_auth();

    // 2. Fetch existing course
    let key = DataKey::Course(course_id);
    let mut course: Course = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(CourseError::CourseNotFound)?;

    // 3. Authorization check (only instructor can update)
    if course.instructor != caller {
        return Err(CourseError::Unauthorized);
    }

    // 4. Update fields if provided
    if let Some(title) = input.title {
        course.title = title;
    }
    if let Some(description) = input.description {
        course.description = description;
    }
    if let Some(category) = input.category {
        course.category = category;
    }
    if let Some(difficulty) = input.difficulty {
        course.difficulty = difficulty;
    }
    if let Some(thumbnail) = input.thumbnail {
        course.thumbnail = thumbnail;
    }
    if let Some(published) = input.published {
        course.published = published;
    }

    // 5. Update timestamp
    course.updated_at = env.ledger().timestamp();

    // 6. Persist updated course
    env.storage().persistent().set(&key, &course);

    Ok(course)
}