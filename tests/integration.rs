#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env};
use vero_core_contracts::{VeroOracle, VeroOracleClient};

fn setup<'a>(env: &'a Env) -> (VeroOracleClient<'a>, Address) {
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VeroOracle);
    let client = VeroOracleClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.init(&admin);
    (client, admin)
}

#[test]
fn test_full_vote_cycle() {
    let env = Env::default();
    let (client, _) = setup(&env);

    let g1 = Address::generate(&env);
    let g2 = Address::generate(&env);
    let g3 = Address::generate(&env);

    client.add_guardian(&g1);
    client.add_guardian(&g2);
    client.add_guardian(&g3);
    client.register_task(&42u64);

    client.vote(&g1, &42u64);
    client.vote(&g2, &42u64);
    client.vote(&g3, &42u64);

    let task = client.get_task(&42u64);
    assert!(task.resolved);
    assert_eq!(task.votes_for, 3);
}

#[test]
#[should_panic(expected = "Not a Guardian")]
fn test_non_guardian_cannot_vote() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let stranger = Address::generate(&env);
    client.register_task(&1u64);
    client.vote(&stranger, &1u64);
}

#[test]
#[should_panic(expected = "Already resolved")]
fn test_no_double_resolve() {
    let env = Env::default();
    let (client, _) = setup(&env);

    let guardians: Vec<Address> = (0..4).map(|_| Address::generate(&env)).collect();
    for g in &guardians {
        client.add_guardian(g);
    }
    client.register_task(&7u64);
    client.vote(&guardians[0], &7u64);
    client.vote(&guardians[1], &7u64);
    client.vote(&guardians[2], &7u64); // resolves
    client.vote(&guardians[3], &7u64); // should panic
}

#[test]
#[should_panic(expected = "Task already exists")]
fn test_no_duplicate_task() {
    let env = Env::default();
    let (client, _) = setup(&env);
    client.register_task(&5u64);
    client.register_task(&5u64);
}
