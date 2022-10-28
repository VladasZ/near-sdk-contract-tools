#![cfg(not(windows))]

use near_sdk::{json_types::Base64VecU8, serde_json::json};
use workspaces::{Account, Contract};

const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/first.wasm");

const NEW_WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/second.wasm");

struct Setup {
    pub contract: Contract,
    pub accounts: Vec<Account>,
}

/// Setup for individual tests
async fn setup(num_accounts: usize, wasm: &[u8]) -> Setup {
    let worker = workspaces::sandbox().await.unwrap();

    // Initialize user accounts
    let mut accounts = vec![];
    for _ in 0..(num_accounts + 1) {
        accounts.push(worker.dev_create_account().await.unwrap());
    }

    let alice = &accounts[0].clone();

    let contract = alice.deploy(&wasm.to_vec()).await.unwrap().unwrap();
    contract.call("new").transact().await.unwrap().unwrap();

    Setup { contract, accounts }
}

#[tokio::test]
async fn upgrade() {
    let Setup { contract, accounts } = setup(1, WASM).await;

    let alice = &accounts[0];

    let code = Base64VecU8::from(Vec::from(NEW_WASM));

    println!("request");
    let request_id: u32 = alice
        .call(contract.id(), "request")
        .max_gas()
        .args_json(json!({
            "request": {
                "Upgrade": {
                    "code": code,
                },
            },
        }))
        .transact()
        .await
        .unwrap()
        .unwrap()
        .json()
        .unwrap();

    println!("approve");
    alice
        .call(contract.id(), "approve")
        .max_gas()
        .args_json(json!({
            "request_id": request_id,
        }))
        .transact()
        .await
        .unwrap()
        .unwrap();

    println!("execute");
    alice
        .call(contract.id(), "execute")
        .max_gas()
        .args_json(json!({
            "request_id": request_id,
        }))
        .transact()
        .await
        .unwrap()
        .unwrap();

    println!("say_hello");
    let hello: String = alice
        .view(contract.id(), "say_hello", vec![])
        .await
        .unwrap()
        .json()
        .unwrap();

    assert_eq!(hello, "hello!");
}
