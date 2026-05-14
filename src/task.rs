use soroban_sdk::{Address, Env};
use crate::types::{DataKey, Task};
use crate::events::emit_verified;
use crate::guardian::is_guardian;

const VOTE_THRESHOLD: u32 = 3;

pub fn register(env: &Env, github_id: u64) {
    assert!(
        !env.storage().instance().has(&DataKey::Task(github_id)),
        "Task already exists"
    );
    let task = Task { github_id, votes_for: 0, resolved: false };
    env.storage().instance().set(&DataKey::Task(github_id), &task);
}

pub fn vote(env: &Env, guardian: Address, github_id: u64) {
    guardian.require_auth();
    assert!(is_guardian(env, &guardian), "Not a Guardian");

    let mut task: Task = env
        .storage()
        .instance()
        .get(&DataKey::Task(github_id))
        .unwrap();
    assert!(!task.resolved, "Already resolved");

    task.votes_for += 1;
    if task.votes_for >= VOTE_THRESHOLD {
        task.resolved = true;
        emit_verified(env, github_id);
    }
    env.storage().instance().set(&DataKey::Task(github_id), &task);
}
