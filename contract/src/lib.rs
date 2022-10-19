/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, BorshStorageKey, StorageUsage};
use near_sdk::{log, near_bindgen, AccountId};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

#[derive(Debug, BorshStorageKey, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    AccountLookupMapU64,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
    pub(crate) account_lookup_map_u64: LookupMap<AccountId, u64>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            message: DEFAULT_MESSAGE.to_string(),
            account_lookup_map_u64: LookupMap::new(StorageKey::AccountLookupMapU64),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn create_account_lookup_map_u64_item(&mut self, user: AccountId) {
        self.account_lookup_map_u64.insert(key, value)
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn get_storage_usage(&self) -> StorageUsage {
        env::storage_usage()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello".to_string());
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy".to_string());
    }
}
