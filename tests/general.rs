use near_sdk_sim::{call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount};

extern crate unit_testing;
use unit_testing::ContractContract as Contract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    TOKEN_WASM_BYTES => "./res/unit_testing.wasm",
}

const SOME_ID: &str = "unit_testing";

pub fn init_one(initial_balance: u128) -> (UserAccount, ContractAccount<Contract>) {
    let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
    genesis.gas_price = 0;
    genesis.gas_limit = u64::MAX;
    let master_account = init_simulator(Some(genesis));
    // uses default values for deposit and gas
    let contract = deploy!(
      // Contract Proxy
      contract: Contract,
      // Contract account id
      contract_id: SOME_ID,
      // Bytes of contract
      bytes: &TOKEN_WASM_BYTES,
      // User deploying the contract,
      signer_account: master_account,
      // init method
      init_method: new(
          master_account.account_id(),
          initial_balance
      )
    );

    (master_account, contract)
}

#[test]
fn first_test() {
    let initial_balance = to_yocto("0");
    let (root, contract) = init_one(initial_balance);

    let outcome = call!(root, contract.say_hello(), deposit = 1);
    outcome.assert_success();

    assert_eq!(outcome.logs()[0], "Hello world");
}
