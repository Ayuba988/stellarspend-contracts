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