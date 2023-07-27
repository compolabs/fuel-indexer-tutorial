# How to Index Data on the Fuel Network using Fuel Indexer

ℹ️ If you have any questions you can ask them in our telegram chat for developers: 
https://t.me/compolabs_devs.

Welcome to an advanced tutorial by Composability Labs on how to index events on the blockchain fuel.network using the fuel indexer.

You can find the source code for this tutorial here: [fuel-indexer-tutorial](https://github.com/compolabs/fuel-indexer-tutorial/tree/master)

⚠️ Before proceeding, please ensure that you are using the beta-3 version of the toolchain. You can switch your toolchain to beta-3 using the following commands:

```bash
fuelup update
fuelup toolchain install beta-3   
fuelup default beta-3
```

## Counter Contract Test Project

1. Create a test counter project:
```bash
forc new counter    
cargo generate --init fuellabs/sway templates/sway-test-rs --name counter
```

2. Add your beta-3 account's private key with some ETH balance (to pay fees) into a `.env` file at the root of the `counter` folder.

Example: [.env.example](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter/.env.example)

3. Change the Rust SDK version in the `Cargo.toml` to 0.41.1 to allow usage of beta-3. Your `Cargo.toml` should look like this: [Cargo.toml](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter/Cargo.toml)

4. The code for the contract `src/main.sw` can be found here: [main.sw](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter/src/main.sw)

5. The test code can be found in `tests/harness.rs`: [harness.rs](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter/tests/harness.rs). This test contains both local node and testnet tests, with the latter used to generate events.

6. Build the contract using `forc build`.

Done! The code is ready. Now let's deploy our contracts. Alternatively, you can use the already deployed contract address on beta-3:

0x.. Address: `0xd968d8c91c8f7a4b09e86fb5080a1cb9bcd8b016f2764751e1c900d2be20b0cc`
fuel.. Address: `fuel1m95d3jgu3aaykz0gd76sszsuhx7d3vqk7fmyw50peyqd903qkrxq67czu2`

---
## Indexer Instance Test Project

1. Create a new indexer instance near the `counter` folder:
```bash
forc index new counter_indexer --namespace composabilitylabs   
cd counter_indexer
```

2. First, copy the ABI file from `counter/out/debug/counter-abi.json` into `counter_indexer/`.

3. Edit the `counter_indexer/schema/counter_indexer.schema.graphql` file to match the following schema: [counter_indexer.schema.graphql](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter_indexer/schema/counter_indexer.schema.graphql)

4. Create a `counter_indexer/src/lib.rs` file like this: [lib.rs](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter_indexer/src/lib.rs)

5. Create the `counter_indexer/counter_indexer.manifest.yaml` manifest as shown here: [counter_indexer.manifest.yaml](https://github.com/compolabs/fuel-indexer-tutorial/blob/master/counter_indexer/counter_indexer.manifest.yaml)

6. If you want to avoid waiting for a long sync, you can change the `start_block` at the end from the fuel explorer.

7. Run the database in a Docker container:
```bash
docker run -d -p 5432:5432 --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword postgres
```

8. Run the fuel-indexer:
```bash
fuel-indexer run --run-migrations --fuel-node-host beta-3.fuel.network --fuel-node-port 80 --postgres-host 127.0.0.1 --postgres-port 5432 --postgres-password mysecretpassword --postgres-user postgres
```

9. Deploy the indexer:
```bash
forc index deploy
```

Done! Now you will see some logs like `INFO fuel_indexer::ffi: 55: Increment 6 -> 7` in your fuel-indexer shell.

To redeploy the indexer code, you can use the following commands:
```bash
forc index remove
forc index deploy
```

Additionally, the GraphQL playground is available at the URL: [http://localhost:29987/api/playground/compolabs/spark_indexer](http://localhost:29987/api/playground/compolabs/spark_indexer)