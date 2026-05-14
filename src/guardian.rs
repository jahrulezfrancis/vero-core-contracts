use soroban_sdk::{Address, Env};
use crate::types::DataKey;

pub fn add(env: &Env, guardian: Address) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();
    env.storage()
        .instance()
        .set(&DataKey::Guardian(guardian), &true);
}

pub fn is_guardian(env: &Env, guardian: &Address) -> bool {
    env.storage()
        .instance()
        .has(&DataKey::Guardian(guardian.clone()))
}
