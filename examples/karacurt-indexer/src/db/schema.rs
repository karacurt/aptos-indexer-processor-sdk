// @generated automatically by Diesel CLI.

diesel::table! {
    awesome_events (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        #[max_length = 66]
        user_address -> Varchar,
        random_number -> Int8,
        timestamp_microseconds -> Int8,
        message -> Text,
        call_count -> Int8,
        transaction_block_height -> Int8,
        event_type -> Text,
        indexed_at -> Timestamp,
    }
}
