use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub github_id: u64,
    pub votes_for: u32,
    pub resolved: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Guardian(Address),
    Task(u64),
}
