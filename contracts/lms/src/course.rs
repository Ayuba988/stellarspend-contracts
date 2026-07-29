use soroban_sdk::{contracterror, contracttype, symbol_short, Address, Env, String, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CourseError {
    CourseNotFound = 1,
    Unauthorized = 2,
    InvalidInput = 3,
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
pub enum DataKey {
    Course(u64),
    NextCourseId,
}

/// Helper function to validate non-empty Soroban strings
fn validate_string(s: &String) -> bool {
    s.len() > 0
}

pub fn create_course(
    env: Env,
    instructor: Address,
    title: String,
    description: String,
    category: String,
    difficulty: u32,
    thumbnail: String,
) -> Result<u64, CourseError> {
    // 1. Authorization check
    instructor.require_auth();

    // 2. Field validations
    if !validate_string(&title)
        || !validate_string(&description)
        || !validate_string(&category)
        || !validate_string(&thumbnail)
    {
        return Err(CourseError::InvalidInput);
    }

    // 3. ID Generation (Auto-increment counter)
    let id_key = DataKey::NextCourseId;
    let course_id: u64 = env.storage().persistent().get(&id_key).unwrap_or(1);
    env.storage().persistent().set(&id_key, &(course_id + 1));

    // 4. Construct Course metadata
    let now = env.ledger().timestamp();
    let course = Course {
        id: course_id,
        instructor: instructor.clone(),
        title: title.clone(),
        description,
        category,
        difficulty,
        thumbnail,
        published: true,
        created_at: now,
        updated_at: now,
    };

    // 5. Store Course metadata
    let course_key = DataKey::Course(course_id);
    env.storage().persistent().set(&course_key, &course);

    // 6. Emit Event: topics -> (Symbol("course"), Symbol("created"), course_id), data -> (instructor, title)
    env.events().publish(
        (symbol_short!("course"), symbol_short!("created"), course_id),
        (instructor, title),
    );

    Ok(course_id)
}