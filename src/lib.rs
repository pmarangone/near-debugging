use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, log, near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    placeholder: u128,
}

#[ext_contract(ext_self)]
pub trait ExtContract {
    fn say_hello(&self);
}

#[near_bindgen]
impl Contract {
    /// Function that initialize the contract.
    ///
    /// Arguments:
    ///
    /// - `owner_id` - the account id that owns the contract
    /// - `placeholder` - symbolic number to demonstrate error when using `deploy!` near-sdk-sim
    #[init]
    pub fn new(owner_id: AccountId, placeholder: u128) -> Self {
        Self {
            owner_id: owner_id,
            placeholder,
        }
    }

    pub fn say_hello(&self) {
        log!("Hello world");
    }
}
