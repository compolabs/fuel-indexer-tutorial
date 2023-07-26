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
