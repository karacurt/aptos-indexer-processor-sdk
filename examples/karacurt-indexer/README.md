# KaracurtTest AwesomeEvent Indexer

This is a custom Aptos indexer that tracks `AwesomeEvent` events emitted by the KaracurtTest smart contract.

## Contract Details

- **Contract Address**: `0xcb91318eca2579aef47ce7bc37e0b62ee7c9f7b27865b938972729e1db6b55eb`
- **Network**: Devnet
- **Event Type**: `karacurt_test::AwesomeEvent`

## Setup

1. **Install PostgreSQL and create database:**
   ```bash
   createdb karacurt_indexer
   ```

2. **Run database migrations:**
   ```bash
   cd src/db
   diesel migration run --database-url postgresql://postgres:@localhost:5432/karacurt_indexer
   ```

3. **Update configuration:**
   - Edit `config.yaml` to set your auth token and database connection
   - Adjust `starting_version` if needed

## Running the Indexer

From the `examples/` directory:

```bash
cargo run -p karacurt-indexer -- -c karacurt-indexer/config.yaml
```

## Database Schema

The indexer tracks the following data in the `awesome_events` table:

- `transaction_version`: Blockchain transaction version
- `event_index`: Event index within the transaction
- `user_address`: Address that triggered the event
- `random_number`: Pseudo-random number from the contract
- `timestamp_microseconds`: Event timestamp
- `message`: Message string from the event
- `call_count`: Number of times the user has called the contract
- `transaction_block_height`: Block height
- `event_type`: Full event type string
- `indexed_at`: When the event was indexed

## Testing the Contract

You can emit test events using the Aptos CLI:

```bash
# Emit a simple AwesomeEvent
aptos move run --function-id 0xcb91318eca2579aef47ce7bc37e0b62ee7c9f7b27865b938972729e1db6b55eb::karacurt_test::emit_simple_awesome_event

# Emit multiple events at once
aptos move run --function-id 0xcb91318eca2579aef47ce7bc37e0b62ee7c9f7b27865b938972729e1db6b55eb::karacurt_test::emit_multiple_awesome_events --args u64:5
```

## Querying the Data

Once the indexer is running, you can query the database:

```sql
-- See all AwesomeEvents
SELECT * FROM awesome_events ORDER BY transaction_version DESC;

-- Count events per user
SELECT user_address, COUNT(*) as event_count 
FROM awesome_events 
GROUP BY user_address;

-- Recent events with their messages
SELECT user_address, message, call_count, indexed_at 
FROM awesome_events 
ORDER BY indexed_at DESC 
LIMIT 10;
```
