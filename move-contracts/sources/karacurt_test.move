module karacurt_test::karacurt_test {
    use aptos_framework::event;
    use aptos_framework::timestamp;
    use std::signer;
    
    // Event emitted when something awesome happens
    #[event]
    struct AwesomeEvent has drop, store {
        user: address,
        random_number: u64,  // Will use counter instead of randomness for now
        timestamp: u64,
        message: vector<u8>,
        call_count: u64,
    }

    // Counter to track how many times the function has been called
    struct EventCounter has key {
        count: u64,
    }

    // Initialize the module - sets up the counter
    fun init_module(account: &signer) {
        move_to(account, EventCounter { count: 0 });
    }

    // Main function that emits an AwesomeEvent with a pseudo-random number (using counter)
    public entry fun emit_awesome_event(account: &signer, message: vector<u8>) acquires EventCounter {
        let user_address = signer::address_of(account);
        
        // Get or initialize counter
        if (!exists<EventCounter>(user_address)) {
            move_to(account, EventCounter { count: 0 });
        };
        
        let counter = borrow_global_mut<EventCounter>(user_address);
        counter.count = counter.count + 1;
        
        // Use counter as pseudo-random number for now
        let pseudo_random = counter.count * 1234567 + 987654321;
        
        // Get current timestamp
        let current_timestamp = timestamp::now_microseconds();
        
        // Emit the awesome event
        event::emit(AwesomeEvent {
            user: user_address,
            random_number: pseudo_random,
            timestamp: current_timestamp,
            message,
            call_count: counter.count,
        });
    }

    // View function to get the current count for an address
    #[view]
    public fun get_count(addr: address): u64 acquires EventCounter {
        if (exists<EventCounter>(addr)) {
            borrow_global<EventCounter>(addr).count
        } else {
            0
        }
    }

    // Simple version that just emits with a default message
    public entry fun emit_simple_awesome_event(account: &signer) acquires EventCounter {
        emit_awesome_event(account, b"Hello from KaracurtTest!");
    }

    // Batch emit multiple events at once
    public entry fun emit_multiple_awesome_events(
        account: &signer, 
        count: u64
    ) acquires EventCounter {
        let i = 0;
        while (i < count) {
            let message = b"Batch event";
            emit_awesome_event(account, message);
            i = i + 1;
        };
    }
} 