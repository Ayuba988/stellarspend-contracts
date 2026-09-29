//! Learner statistics derived from enrollment and course progress records.

use crate::{storage, types::StudentStatistics};
use soroban_sdk::{Address, Env};

/// Returns enrollment, completion, lesson, XP, badge, and certificate totals.
pub fn student_statistics(env: &Env, student: &Address) -> StudentStatistics {
    let Some(record) = storage::read_student(env, student) else {
        return StudentStatistics::default();
    };

    let mut lessons_completed = 0_u64;
    for course_id in record.enrolled_courses.iter() {
        if let Some(progress) = storage::read_progress(env, student, course_id) {
            lessons_completed += progress.completed_lessons.len() as u64;
        }
    }

    StudentStatistics {
        courses_enrolled: record.enrolled_courses.len() as u64,
        courses_completed: record.completed_courses.len() as u64,
        lessons_completed,
        xp: record.xp,
        badges: record.badges,
        certificates: record.completed_courses.len() as u64,
    }
}