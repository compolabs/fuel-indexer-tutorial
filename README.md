# Fuel indexer tutorial

Switch to the latest version of the indexer.
```
fuelup update
fuelup toolchain install latest   
fuelup default latest
```


## Create a test counter project
```
forc new counter    
cargo generate --init fuellabs/sway templates/sway-test-rs --name counter
```

Change rust SDK version in the `Cargo.toml` to 0.41.1 to allow usage of beta-3

---
**`Cargo.toml`**
```
[package]
name = "counter"
description = "A cargo-generate template for Rust + Sway integration testing."
version = "0.1.0"
edition = "2021"
authors = ["alexey <alexnagornnyy.an@gmail.com>"]
license = "Apache-2.0"

[dev-dependencies]
fuels = { version = "0.41.1", features = ["fuel-core-lib"] }
tokio = { version = "1.12", features = ["rt", "macros"] }

[[test]]
harness = true
name = "integration_tests"
path = "tests/harness.rs"
```
---
**`main.sw`**

```rs
contract; 
use std::block::timestamp;

storage {   
    counter: u64 = 0,
}
 
abi Counter {
    #[storage(read, write)]
    fn increment();

    #[storage(read)]
    fn count() -> u64;
}

struct IncrementEvent {
    last_counter: u64,
    new_counter: u64,
    timestamp: u64
}


impl Counter for Contract { 
    #[storage(read)]
    fn count() -> u64 {
        storage.counter.try_read().unwrap_or(0)
    }

    #[storage(read, write)]
    fn increment() {
        let last_counter = storage.counter.try_read().unwrap_or(0);
        let new_counter = last_counter + 1;
        
        storage.counter.write(new_counter);
        log(IncrementEvent{
            last_counter,
            new_counter,
            timestamp: timestamp()
        })
    }
}
```
---
**`harness.rs`**

```rs
use fuels::{prelude::*, tx::ContractId};

abigen!(Contract(
    name = "Counter",
    abi = "out/debug/counter-abi.json"
));

async fn get_contract_instance() -> (Counter<WalletUnlocked>, ContractId) {
    let config = WalletsConfig::new(Some(1), Some(1), Some(1_000_000_000));
    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from("./out/debug/counter.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = Counter::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn local_test() {
    let (instance, _id) = get_contract_instance().await;
    let count = instance.methods().count().simulate().await.unwrap().value;
    assert!(count == 0);
    instance.methods().increment().call().await.unwrap();
    let count = instance.methods().count().simulate().await.unwrap().value;
    assert!(count == 1);
}

```
---
## Create a new indexer instance
```
forc index new counter-indexer --namespace composabilitylabs
cd hello_world
```
