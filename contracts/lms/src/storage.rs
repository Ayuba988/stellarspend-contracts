use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    /// Storage key for a specific course by ID
    Course(u64),
    /// Storage key for a specific lesson by ID
    Lesson(u64),
    /// Storage key for a specific module by ID
    Module(u64),
    /// Storage key for a specific quiz by ID
    Quiz(u64),
    /// Storage key for student account record by Address
    Student(Address),
    /// Storage key for a certificate record by ID or serial string
    Certificate(String),
    /// Storage key tracking student progress for a given course (Student Address, Course ID)
    Progress(Address, u64),
}