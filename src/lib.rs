//Deposit tokens into the vault
//Withdrawal of tokens from vault
//Time constraints for withdrawal
//Beneficiary and owner authentication
//Emergency withdrawal by ownwer

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contract]
pub struct TimeLockedVault;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owner,
    Beneficiary,
    UnlockTime,
    Token,
    DepositedAmount,
}

#[contractimpl]
impl TimeLockedVault {
    pub fn initialize(
        env: Env,
        owner: Address,
        beneficiary: Address,
        unlock_time: u64,
        token: Address,
    ) {
        if owner == beneficiary {
            panic!("Owner and beneficiary cannot be the same");
        }
        if unlock_time <= env.ledger().timestamp() {
            panic!("Unlock time must be in the future");
        }

        env.storage().persistent().set(&DataKey::Owner, &owner);
        env.storage().persistent().set(&DataKey::Beneficiary, &beneficiary);
        env.storage().persistent().set(&DataKey::UnlockTime, &unlock_time);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage().persistent().set(&DataKey::DepositedAmount, &0);
    }

    pub fn deposit(env: Env, from: Address, amount: i128) {
        if amount <= 0 {
            panic!("Deposit amount must be positive");
        }

        from.require_auth();

        let owner: Address = env.storage().persistent().get(&DataKey::Owner).unwrap();
        if from != owner {
            panic!("Only owner can deposit");
        }

        let token: Address = env.storage().persistent().get(&DataKey::Token).unwrap();
        let token_client = soroban_sdk::token::Client::new(&env, &token);
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        let current_amount: i128 = env.storage().persistent().get(&DataKey::DepositedAmount).unwrap();
        env.storage().persistent().set(&DataKey::DepositedAmount, &(current_amount + amount));
    }

    pub fn withdraw(env: Env) {
        let beneficiary: Address = env.storage().persistent().get(&DataKey::Beneficiary).unwrap();
        beneficiary.require_auth();

        let unlock_time: u64 = env.storage().persistent().get(&DataKey::UnlockTime).unwrap();
        if env.ledger().timestamp() < unlock_time {
            panic!("Funds are still locked");
        }

        let token: Address = env.storage().persistent().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().persistent().get(&DataKey::DepositedAmount).unwrap();

        if amount == 0 {
            panic!("No funds to withdraw");
        }

        let token_client = soroban_sdk::token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &beneficiary, &amount);

        env.storage().persistent().set(&DataKey::DepositedAmount, &0);
    }

    pub fn emergency_withdraw(env: Env) {
        let owner: Address = env.storage().persistent().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let token: Address = env.storage().persistent().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().persistent().get(&DataKey::DepositedAmount).unwrap();

        if amount == 0 {
            panic!("No funds to withdraw");
        }

        let token_client = soroban_sdk::token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &owner, &amount);

        env.storage().persistent().set(&DataKey::DepositedAmount, &0);
    }

    // View functions
    pub fn get_owner(env: Env) -> Address {
        env.storage().persistent().get(&DataKey::Owner).unwrap()
    }

    pub fn get_beneficiary(env: Env) -> Address {
        env.storage().persistent().get(&DataKey::Beneficiary).unwrap()
    }

    pub fn get_unlock_time(env: Env) -> u64 {
        env.storage().persistent().get(&DataKey::UnlockTime).unwrap()
    }

    pub fn get_token(env: Env) -> Address {
        env.storage().persistent().get(&DataKey::Token).unwrap()
    }

    pub fn get_deposited_amount(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::DepositedAmount).unwrap()
    }
}