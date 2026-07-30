use soroban_sdk::{contracttype, Env};

/// Storage keys used for dashboard counters.
/// Replace these with your project's existing storage keys if they differ.
#[contracttype]
#[derive(Clone)]
pub enum AnalyticsKey {
    CourseCount,
    LessonCount,
    StudentCount,
    CertificateCount,
    RewardCount,
    TotalXpIssued,
}

/// Dashboard summary returned to the frontend.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardSummary {
    pub courses: u32,
    pub lessons: u32,
    pub students: u32,
    pub certificates: u32,
    pub rewards: u32,
    pub xp_issued: u64,
}

/// Read a u32 counter from storage.
fn read_u32(env: &Env, key: AnalyticsKey) -> u32 {
    env.storage()
        .instance()
        .get::<AnalyticsKey, u32>(&key)
        .unwrap_or(0)
}

/// Read a u64 counter from storage.
fn read_u64(env: &Env, key: AnalyticsKey) -> u64 {
    env.storage()
        .instance()
        .get::<AnalyticsKey, u64>(&key)
        .unwrap_or(0)
}

/// Returns a single dashboard summary containing all LMS statistics.
pub fn get_dashboard_summary(env: &Env) -> DashboardSummary {
    DashboardSummary {
        courses: read_u32(env, AnalyticsKey::CourseCount),
        lessons: read_u32(env, AnalyticsKey::LessonCount),
        students: read_u32(env, AnalyticsKey::StudentCount),
        certificates: read_u32(env, AnalyticsKey::CertificateCount),
        rewards: read_u32(env, AnalyticsKey::RewardCount),
        xp_issued: read_u64(env, AnalyticsKey::TotalXpIssued),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn dashboard_should_return_zero_when_empty() {
        let env = Env::default();

        let summary = get_dashboard_summary(&env);

        assert_eq!(summary.courses, 0);
        assert_eq!(summary.lessons, 0);
        assert_eq!(summary.students, 0);
        assert_eq!(summary.certificates, 0);
        assert_eq!(summary.rewards, 0);
        assert_eq!(summary.xp_issued, 0);
    }

    #[test]
    fn dashboard_should_return_saved_values() {
        let env = Env::default();

        env.storage()
            .instance()
            .set(&AnalyticsKey::CourseCount, &5u32);

        env.storage()
            .instance()
            .set(&AnalyticsKey::LessonCount, &18u32);

        env.storage()
            .instance()
            .set(&AnalyticsKey::StudentCount, &42u32);

        env.storage()
            .instance()
            .set(&AnalyticsKey::CertificateCount, &11u32);

        env.storage()
            .instance()
            .set(&AnalyticsKey::RewardCount, &7u32);

        env.storage()
            .instance()
            .set(&AnalyticsKey::TotalXpIssued, &24500u64);

        let summary = get_dashboard_summary(&env);

        assert_eq!(summary.courses, 5);
        assert_eq!(summary.lessons, 18);
        assert_eq!(summary.students, 42);
        assert_eq!(summary.certificates, 11);
        assert_eq!(summary.rewards, 7);
        assert_eq!(summary.xp_issued, 24500);
    }
}