use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, IntoVal};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    let owner = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let token = Address::generate(&env);
    let unlock_time = env.ledger().timestamp() + 1000;

    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &beneficiary, &unlock_time, &token);

    assert_eq!(client.get_owner(), owner);
    assert_eq!(client.get_beneficiary(), beneficiary);
    assert_eq!(client.get_unlock_time(), unlock_time);
    assert_eq!(client.get_token(), token);
    assert_eq!(client.get_deposited_amount(), 0);
}

#[test]
#[should_panic(expected = "Deposit amount must be positive")]
fn test_deposit_negative_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    let owner = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let token = Address::generate(&env);

    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &beneficiary, &1000, &token);
    client.deposit(&owner, &-1);
}

#[test]
fn test_deposit_and_withdraw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    // Setup test accounts
    let owner = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let token_admin = Address::generate(&env);
    
    // Create test token
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = token::Client::new(&env, &token_id);
    
    // Mint tokens to owner
    token_admin.require_auth();
    token_client.mint(&owner, &1000);

    // Initialize vault with unlock time 100 seconds in future
    let unlock_time = env.ledger().timestamp() + 100;
    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &beneficiary, &unlock_time, &token_id);

    // Deposit tokens
    owner.require_auth();
    client.deposit(&owner, &500);
    assert_eq!(token_client.balance(&contract_id), 500);
    assert_eq!(token_client.balance(&owner), 500);
    assert_eq!(client.get_deposited_amount(), 500);

    // Attempt early withdrawal (should fail)
    assert!(std::panic::catch_unwind(|| {
        beneficiary.require_auth();
        client.withdraw();
    }).is_err());

    // Advance time past unlock
    env.ledger().with_mut(|l| {
        l.timestamp = unlock_time + 1;
    });

    // Withdraw as beneficiary
    beneficiary.require_auth();
    client.withdraw();
    assert_eq!(token_client.balance(&contract_id), 0);
    assert_eq!(token_client.balance(&beneficiary), 500);
    assert_eq!(client.get_deposited_amount(), 0);
}

#[test]
fn test_emergency_withdraw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    let owner = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&env, &token_id);
    
    token_admin.require_auth();
    token_client.mint(&owner, &1000);

    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &beneficiary, &1000, &token_id);

    owner.require_auth();
    client.deposit(&owner, &500);
    assert_eq!(token_client.balance(&contract_id), 500);
    assert_eq!(client.get_deposited_amount(), 500);

    // Emergency withdraw as owner (before unlock time)
    owner.require_auth();
    client.emergency_withdraw();
    assert_eq!(token_client.balance(&contract_id), 0);
    assert_eq!(token_client.balance(&owner), 1000);
    assert_eq!(client.get_deposited_amount(), 0);
}

#[test]
#[should_panic(expected = "Funds are still locked")]
fn test_withdraw_before_unlock() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    let owner = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let token = Address::generate(&env);

    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &beneficiary, &1000, &token);
    client.deposit(&owner, &100);

    beneficiary.require_auth();
    client.withdraw(); // Should panic
}

#[test]
#[should_panic(expected = "Only owner can deposit")]
fn test_unauthorized_deposit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    let owner = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let attacker = Address::generate(&env);
    let token = Address::generate(&env);

    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &beneficiary, &1000, &token);

    attacker.require_auth();
    client.deposit(&attacker, &100); // Should panic
}

#[test]
#[should_panic(expected = "Owner and beneficiary cannot be the same")]
fn test_owner_equals_beneficiary() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TimeLockedVault);

    let owner = Address::generate(&env);
    let token = Address::generate(&env);

    let client = TimeLockedVaultClient::new(&env, &contract_id);
    client.initialize(&owner, &owner, &1000, &token); // Should panic
}