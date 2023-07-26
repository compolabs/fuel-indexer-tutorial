extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "counter_indexer.manifest.yaml")]
pub mod counter_indexer_index_mod {

    fn counter_indexer_handler(block_data: BlockData) {
        info!("Processing Block#{}. (>'.')>", block_data.height);

        let block_id = id8(block_data.id);
        let block = Block{ id: block_id, height: block_data.height, hash: block_data.id };
        block.save();

        for transaction in block_data.transactions.iter() {
            let tx = Transaction{ id: id8(transaction.id), block: block_data.id, hash: Bytes32::from(<[u8; 32]>::from(transaction.id)) };
            tx.save();
        }
    }
}
