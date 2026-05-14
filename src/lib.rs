#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod events;
mod guardian;
mod task;
mod types;

use types::{DataKey, Task};

#[contract]
pub struct VeroOracle;

#[contractimpl]
impl VeroOracle {
    pub fn init(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "Already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn add_guardian(env: Env, guardian: Address) {
        guardian::add(&env, guardian);
    }

    pub fn register_task(env: Env, github_id: u64) {
        task::register(&env, github_id);
    }

    pub fn vote(env: Env, guardian: Address, github_id: u64) {
        task::vote(&env, guardian, github_id);
    }

    pub fn get_task(env: Env, github_id: u64) -> Task {
        env.storage()
            .instance()
            .get(&DataKey::Task(github_id))
            .unwrap()
    }
}
