use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{log, near_bindgen};
use uint::construct_uint;

const VALUE: u128 = 199999999999999999999;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn pprint_u128(&self) -> u128 {
        let x: u128 = 199999579045383882874;
        let y: u128 = 429189948917157334187;
        let z: u128 = 429190852266509618340;

        let value: u128 = (U256::from(x) * U256::from(z) / U256::from(y)).as_u128();
        let value_str: U128 = U128((U256::from(x) * U256::from(z) / U256::from(y)).as_u128());

        log!("u128: {}", value);
        log!("U128: {:?}", value_str);

        value
    }

    pub fn pprint_str(&self) -> U128 {
        let x: u128 = 199999579045383882874;
        let y: u128 = 429189948917157334187;
        let z: u128 = 429190852266509618340;

        let value: u128 = (U256::from(x) * U256::from(z) / U256::from(y)).as_u128();
        let value_str: U128 = U128((U256::from(x) * U256::from(z) / U256::from(y)).as_u128());

        log!("u128: {}", value);
        log!("U128: {:?}", value_str);

        value_str
    }

    pub fn default_u128(&self) -> u128 {
        let value_str: U128 = U128(VALUE);

        log!("u128: {}", VALUE);
        log!("U128: {:?}", value_str);

        VALUE
    }

    pub fn default_str(&self) -> U128 {
        let value_str: U128 = U128(VALUE);

        log!("u128: {}", VALUE);
        log!("U128: {:?}", value_str);

        value_str
    }
}
