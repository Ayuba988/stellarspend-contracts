#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_storage_keys_read_write() {
        let env = Env::default();
        let student_addr = Address::generate(&env);
        let cert_id = String::from_str(&env, "CERT-2026-001");

        // Define test instances for all 7 required variants
        let keys = vec![
            &env,
            StorageKey::Course(101),
            StorageKey::Lesson(202),
            StorageKey::Module(303),
            StorageKey::Quiz(404),
            StorageKey::Student(student_addr.clone()),
            StorageKey::Certificate(cert_id.clone()),
            StorageKey::Progress(student_addr.clone(), 101),
        ];

        // Verify write, exists, and read back for each key
        for (i, key) in keys.iter().enumerate() {
            let dummy_val = (i + 1) as u64;
            
            // Set value in instance storage
            env.storage().instance().set(key, &dummy_val);

            // Verify existence and equality
            assert!(env.storage().instance().has(key));
            let retrieved: u64 = env.storage().instance().get(key).unwrap();
            assert_eq!(retrieved, dummy_val);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_publish_draft_course_success() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let course_id = 1u64;

        let course = Course {
            id: course_id,
            admin: admin.clone(),
            title: String::from_str(&env, "Soroban 101"),
            published: false,
        };

        let key = StorageKey::Course(course_id);
        env.storage().instance().set(&key, &course);

        // Publish course with admin authorization
        env.mock_all_auths();
        let result = publish_course(env.clone(), admin.clone(), course_id);
        assert!(result.is_ok());

        // Verify published state updated
        let updated: Course = env.storage().instance().get(&key).unwrap();
        assert!(updated.published);
    }

    #[test]
    fn test_publish_invalid_course_fails() {
        let env = Env::default();
        let caller = Address::generate(&env);

        env.mock_all_auths();
        let result = publish_course(env, caller, 999u64);
        assert_eq!(result, Err(CourseError::CourseNotFound));
    }
}