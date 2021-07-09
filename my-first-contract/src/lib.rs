use near_sdk::{near_bindgen, env};
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct MyFirstContract {
    string_hash_map: HashMap<String, String>
}
