#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, token::Client as TokenClient, Env};

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotAuthorizedToWithdraw = 1,
    NegativeAmount = 2,
    TimePredicateUnfulfilled = 3,
    NoReceiptsFound = 4,
}

#[contracttype]
pub enum StorageKey {
    /// Admin. Value is an Address.
    Admin,
    /// A receipt is keyed by the recipient address, and receipt count.
    /// Value is a ReceiptConfig.
    Receipt(Address, u32),
    ReceiptCount(Address),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptConfig {
    amount: i128,
    depositor: Address,
    token: Address,
    time_bound: TimeBound,
}

#[contract]
pub struct EscrowContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl EscrowContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&StorageKey::Admin, &admin);
    }
    
    /// Set the admin.
    pub fn set_admin(env: Env, new_admin: Address) {
        Self::admin(env.clone()).require_auth();
        env.storage().instance().set(&StorageKey::Admin, &new_admin);
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&StorageKey::Admin)
            .unwrap()
    }


    pub fn deposit(env: Env, depositor: Address, recipient: Address, token: Address, amount: i128, time_bound: TimeBound) -> Result<(ReceiptConfig, u32), Error> {
        // require auth
        depositor.require_auth();

        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }
        
        // check state
        let index: u32 = env
            .storage()
            .instance()
            .get(&StorageKey::ReceiptCount(recipient.clone()))
            .unwrap_or(0u32) + 1;

        let storage_key = &StorageKey::Receipt(recipient.clone(), index);

        // move tokens to smart contract
        let token_client = TokenClient::new(&env, &token);
        let contract_address: Address = env.current_contract_address();
        token_client.transfer(&depositor, &contract_address, &amount);

        // update state
        let receipt = ReceiptConfig {
            amount,
            token,
            time_bound,
            depositor,
        };
        env.storage().instance().set::<_, ReceiptConfig>(storage_key, &receipt);
        env.storage().instance().set::<_, u32>(&StorageKey::ReceiptCount(recipient.clone()), &index);

        let epoch = env.ledger().sequence();
        Ok((receipt, epoch))

    }

    pub fn withdraw(env: Env, recipient: Address, index: u32) -> Result<(ReceiptConfig, u32), Error> {
        recipient.require_auth();

        // check state
        let storage_key = &StorageKey::Receipt(recipient.clone(), index);
        let receipt = env
            .storage()
            .instance()
            .get::<_, ReceiptConfig>(storage_key)
            .ok_or(Error::NoReceiptsFound)?;

        if !check_time_bound(&env, &receipt.time_bound) {
            return Err(Error::TimePredicateUnfulfilled);
        }

        // move tokens from smart contract
        let token_client = TokenClient::new(&env, &receipt.token);
        let contract_address: Address = env.current_contract_address();
        token_client.transfer(&contract_address, &recipient, &receipt.amount);

        env.storage().instance().remove(storage_key);
        let epoch = env.ledger().sequence();
        Ok((receipt, epoch))

    }
}

mod test;
