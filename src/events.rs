use soroban_sdk::{Env, Symbol};

pub fn emit_verified(env: &Env, github_id: u64) {
    env.events()
        .publish((Symbol::new(env, "verified"), github_id), github_id);
}
