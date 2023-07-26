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