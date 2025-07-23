use crate::schema::awesome_events;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::Event as EventPB,
    utils::convert::standardize_address,
};
use diesel::{Identifiable, Insertable, Queryable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(primary_key(transaction_version, event_index))]
#[diesel(table_name = awesome_events)]
pub struct AwesomeEvent {
    pub transaction_version: i64,
    pub event_index: i64,
    pub user_address: String,
    pub random_number: i64,
    pub timestamp_microseconds: i64,
    pub message: String,
    pub call_count: i64,
    pub transaction_block_height: i64,
    pub event_type: String,
    pub indexed_at: chrono::NaiveDateTime,
}

impl AwesomeEvent {
    pub fn from_event(
        event: &EventPB,
        txn_version: i64,
        event_index: i64,
        block_height: i64,
    ) -> anyhow::Result<Self> {
        let event_data: Value = serde_json::from_str(&event.data)?;
        
        let user_address = standardize_address(&event_data["user"].as_str().unwrap_or_default());
        let random_number = event_data["random_number"].as_str()
            .unwrap_or("0")
            .parse::<i64>()
            .unwrap_or(0);
        let timestamp_microseconds = event_data["timestamp"].as_str()
            .unwrap_or("0")
            .parse::<i64>()
            .unwrap_or(0);
        let call_count = event_data["call_count"].as_str()
            .unwrap_or("0")
            .parse::<i64>()
            .unwrap_or(0);
        
        let message_bytes = event_data["message"].as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_u64())
                    .map(|v| v as u8)
                    .collect::<Vec<u8>>()
            })
            .unwrap_or_default();
        let message = String::from_utf8(message_bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string());

        Ok(AwesomeEvent {
            transaction_version: txn_version,
            event_index,
            user_address,
            random_number,
            timestamp_microseconds,
            message,
            call_count,
            transaction_block_height: block_height,
            event_type: event.type_str.clone(),
            indexed_at: chrono::Utc::now().naive_utc(),
        })
    }
    
    pub fn is_awesome_event(event: &EventPB) -> bool {
        event.type_str.contains("karacurt_test::AwesomeEvent")
    }
}
