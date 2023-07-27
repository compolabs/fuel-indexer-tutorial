use dotenv::dotenv;
use std::{env, str::FromStr};

use fuels::{prelude::*, tx::ContractId};

abigen!(Contract(
    name = "Counter",
    abi = "out/debug/counter-abi.json"
));

const RPC: &str = "beta-3.fuel.network";
const ADDRESS: &str = "0xd968d8c91c8f7a4b09e86fb5080a1cb9bcd8b016f2764751e1c900d2be20b0cc";
#[tokio::test]
async fn testnet_test() {
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();

    let pk = env::var("SECRET").unwrap().parse().unwrap();
    let wallet = WalletUnlocked::new_from_private_key(pk, Some(provider.clone()));
    let contract_id: Bech32ContractId = ContractId::from_str(ADDRESS).unwrap().into();
    println!("{:?}", contract_id.to_string());
    let instance = Counter::new(contract_id, wallet.clone());
    instance
        .methods()
        .increment()
        .tx_params(TxParameters::default().set_gas_price(1))
        .call()
        .await
        .unwrap();
}

#[tokio::test]
async fn local_test() {
    let config = WalletsConfig::new(Some(1), Some(1), Some(1_000_000_000));
    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from("./out/debug/counter.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = Counter::new(id.clone(), wallet);

    let count = instance.methods().count().simulate().await.unwrap().value;
    assert!(count == 0);
    instance.methods().increment().call().await.unwrap();
    let count = instance.methods().count().simulate().await.unwrap().value;
    assert!(count == 1);
}
