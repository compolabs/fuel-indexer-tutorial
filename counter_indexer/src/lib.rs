extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "counter_indexer.manifest.yaml")]
pub mod counter_indexer_index_mod {

    fn counter_indexer_handler(increment_data: IncrementEvent) {
        let increment = Increment {
            id: increment_data.timestamp,
            last_counter: increment_data.last_counter,
            new_counter: increment_data.new_counter,
            timestamp: increment_data.timestamp,
        };
        increment.save();
        info!(
            "Increment {} -> {}",
            increment_data.last_counter, increment_data.new_counter
        );
    }
}
