#![cfg(test)]

use soroban_sdk::{Env, String, Vec};

use crate::{LMSContract, Module};

#[test]
fn test_initialize() {
    let _env = Env::default();

    let result = LMSContract::initialize();

    assert!(result);
}

#[test]
fn test_create_module() {
    let env = Env::default();

    let mut lessons = Vec::new(&env);

    lessons.push_back(1);
    lessons.push_back(2);
    lessons.push_back(3);

    let module = Module {
        module_id: 1,
        course_id: 100,
        title: String::from_str(&env, "Introduction"),
        lesson_ids: lessons.clone(),
        display_order: 1,
    };

    assert_eq!(module.module_id, 1);
    assert_eq!(module.course_id, 100);
    assert_eq!(module.lesson_ids.len(), 3);
    assert_eq!(module.display_order, 1);

    assert_eq!(module.lesson_ids.get(0), Some(1));
    assert_eq!(module.lesson_ids.get(1), Some(2));
    assert_eq!(module.lesson_ids.get(2), Some(3));
}

use soroban_sdk::{Env, String};

use crate::Lesson;

#[test]
fn test_create_lesson() {
    let env = Env::default();

    let lesson = Lesson {
        lesson_id: 1,
        course_id: 100,
        title: String::from_str(&env, "Introduction"),
        description: String::from_str(&env, "Welcome to the course"),
        content_uri: String::from_str(&env, "ipfs://QmLessonHash"),
        estimated_duration: 30,
        lesson_order: 1,
    };

    assert_eq!(lesson.lesson_id, 1);
    assert_eq!(lesson.course_id, 100);
    assert_eq!(lesson.title, String::from_str(&env, "Introduction"));
    assert_eq!(lesson.estimated_duration, 30);
    assert_eq!(lesson.lesson_order, 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_quiz() {
        let quiz = Quiz {
            quiz_id: 1,
            lesson_id: 10,
            passing_score: 70,
            maximum_score: 100,
            reward_points: 50,
            is_active: true,
        };

        assert_eq!(quiz.quiz_id, 1);
        assert_eq!(quiz.lesson_id, 10);
        assert_eq!(quiz.passing_score, 70);
        assert_eq!(quiz.maximum_score, 100);
        assert_eq!(quiz.reward_points, 50);
        assert!(quiz.is_active);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_quiz() {
        let quiz = Quiz {
            quiz_id: 1,
            lesson_id: 10,
            passing_score: 70,
            maximum_score: 100,
            reward_points: 50,
            is_active: true,
        };

        assert_eq!(quiz.quiz_id, 1);
        assert_eq!(quiz.lesson_id, 10);
        assert_eq!(quiz.passing_score, 70);
        assert_eq!(quiz.maximum_score, 100);
        assert_eq!(quiz.reward_points, 50);
        assert!(quiz.is_active);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, testutils::Ledger, Env, String};

    fn setup_test_course(env: &Env, instructor: &Address, course_id: u64) -> Course {
        let course = Course {
            id: course_id,
            instructor: instructor.clone(),
            title: String::from_str(env, "Old Title"),
            description: String::from_str(env, "Old Description"),
            category: String::from_str(env, "Old Category"),
            difficulty: 1,
            thumbnail: String::from_str(env, "https://old.png"),
            published: false,
            created_at: env.ledger().timestamp(),
            updated_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Course(course_id), &course);

        course
    }

    #[test]
    fn test_successful_update() {
        let env = Env::default();
        env.mock_all_signatures();

        let instructor = Address::generate(&env);
        let course_id = 1u64;
        setup_test_course(&env, &instructor, course_id);

        // Advance timestamp to test updated_at change
        env.ledger().set_timestamp(1_000_000);

        let update_input = UpdateCourseInput {
            title: Some(String::from_str(&env, "New Title")),
            description: Some(String::from_str(&env, "New Description")),
            category: None, // Leave unchanged
            difficulty: Some(2),
            thumbnail: None,
            published: Some(true),
        };

        let result = update_course(env.clone(), instructor.clone(), course_id, update_input);
        assert!(result.is_ok());

        let updated_course = result.unwrap();
        assert_eq!(updated_course.title, String::from_str(&env, "New Title"));
        assert_eq!(updated_course.description, String::from_str(&env, "New Description"));
        assert_eq!(updated_course.category, String::from_str(&env, "Old Category"));
        assert_eq!(updated_course.difficulty, 2);
        assert_eq!(updated_course.published, true);
        assert_eq!(updated_course.updated_at, 1_000_000);
    }

    #[test]
    fn test_unauthorized_update_rejected() {
        let env = Env::default();
        env.mock_all_signatures();

        let instructor = Address::generate(&env);
        let unauthorized_user = Address::generate(&env);
        let course_id = 1u64;

        setup_test_course(&env, &instructor, course_id);

        let update_input = UpdateCourseInput {
            title: Some(String::from_str(&env, "Hacked Title")),
            description: None,
            category: None,
            difficulty: None,
            thumbnail: None,
            published: None,
        };

        let result = update_course(env, unauthorized_user, course_id, update_input);
        assert_eq!(result, Err(CourseError::Unauthorized));
    }

    #[test]
    fn test_non_existent_course_returns_error() {
        let env = Env::default();
        env.mock_all_signatures();

        let instructor = Address::generate(&env);
        let non_existent_course_id = 999u64;

        let update_input = UpdateCourseInput {
            title: Some(String::from_str(&env, "Title")),
            description: None,
            category: None,
            difficulty: None,
            thumbnail: None,
            published: None,
        };

        let result = update_course(env, instructor, non_existent_course_id, update_input);
        assert_eq!(result, Err(CourseError::CourseNotFound));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Events, Ledger},
        IntoVal, Env, String,
    };

    #[test]
    fn test_create_course_success() {
        let env = Env::default();
        env.mock_all_signatures();
        env.ledger().set_timestamp(500_000);

        let instructor = Address::generate(&env);

        let title = String::from_str(&env, "Introduction to Soroban");
        let description = String::from_str(&env, "Learn Rust and Stellar smart contracts.");
        let category = String::from_str(&env, "Blockchain");
        let thumbnail = String::from_str(&env, "https://example.com/thumb.png");

        let course_id = create_course(
            env.clone(),
            instructor.clone(),
            title.clone(),
            description.clone(),
            category.clone(),
            1,
            thumbnail.clone(),
        )
        .expect("Course creation should succeed");

        assert_eq!(course_id, 1);

        // Verify state persistent storage
        let stored_course: Course = env
            .storage()
            .persistent()
            .get(&DataKey::Course(course_id))
            .expect("Course should exist in storage");

        assert_eq!(stored_course.id, 1);
        assert_eq!(stored_course.instructor, instructor);
        assert_eq!(stored_course.title, title);
        assert_eq!(stored_course.created_at, 500_000);

        // Verify Event emission
        let events = env.events().all();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_create_course_unique_ids() {
        let env = Env::default();
        env.mock_all_signatures();

        let instructor = Address::generate(&env);
        let title = String::from_str(&env, "Course Title");
        let desc = String::from_str(&env, "Course Description");
        let cat = String::from_str(&env, "Category");
        let thumb = String::from_str(&env, "https://example.com/thumb.png");

        let id_1 = create_course(env.clone(), instructor.clone(), title.clone(), desc.clone(), cat.clone(), 1, thumb.clone()).unwrap();
        let id_2 = create_course(env.clone(), instructor.clone(), title.clone(), desc.clone(), cat.clone(), 2, thumb.clone()).unwrap();

        assert_ne!(id_1, id_2);
        assert_eq!(id_1, 1);
        assert_eq!(id_2, 2);
    }

    #[test]
    fn test_create_course_invalid_input_rejected() {
        let env = Env::default();
        env.mock_all_signatures();

        let instructor = Address::generate(&env);
        let empty_title = String::from_str(&env, "");
        let desc = String::from_str(&env, "Valid Description");
        let cat = String::from_str(&env, "Valid Category");
        let thumb = String::from_str(&env, "https://example.com/thumb.png");

        let result = create_course(
            env.clone(),
            instructor,
            empty_title,
            desc,
            cat,
            1,
            thumb,
        );

        assert_eq!(result, Err(CourseError::InvalidInput));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize_and_get_admin() {
        let env = Env::default();
        env.mock_all_signatures();

        let admin = Address::generate(&env);
        assert!(initialize_admin(env.clone(), admin.clone()).is_ok());

        assert_eq!(get_admin(&env).unwrap(), admin);
        assert_eq!(get_role(&env, &admin), Role::Admin);

        // Second initialization must fail
        assert_eq!(
            initialize_admin(env.clone(), admin),
            Err(AdminError::AlreadyInitialized)
        );
    }

    #[test]
    fn test_authorized_role_assignment() {
        let env = Env::default();
        env.mock_all_signatures();

        let admin = Address::generate(&env);
        let instructor = Address::generate(&env);
        initialize_admin(env.clone(), admin.clone()).unwrap();

        // Admin assigns Instructor role
        let result = set_role(env.clone(), admin, instructor.clone(), Role::Instructor);
        assert!(result.is_ok());
        assert_eq!(get_role(&env, &instructor), Role::Instructor);

        // Instructor passes instructor/admin authorization check
        assert!(require_instructor_or_admin(&env, &instructor).is_ok());
    }

    #[test]
    fn test_unauthorized_action_rejected() {
        let env = Env::default();
        env.mock_all_signatures();

        let admin = Address::generate(&env);
        let student = Address::generate(&env);
        let target = Address::generate(&env);
        initialize_admin(env.clone(), admin).unwrap();

        // Default role is Student
        assert_eq!(get_role(&env, &student), Role::Student);

        // Student attempting to set role must fail
        let result = set_role(env.clone(), student.clone(), target, Role::Instructor);
        assert_eq!(result, Err(AdminError::Unauthorized));

        // Student attempting privileged action must fail
        assert_eq!(
            require_instructor_or_admin(&env, &student),
            Err(AdminError::Unauthorized)
        );
    }
}