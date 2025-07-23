use crate::awesome_event_model::AwesomeEvent;
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::transaction::TxnData,
    postgres::{
        basic_processor::process,
        utils::database::{execute_in_chunks, MAX_DIESEL_PARAM_SIZE},
    },
};
use diesel::{pg::Pg, query_builder::QueryFragment};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use field_count::FieldCount;
use rayon::prelude::*;
use tracing::{error, info, warn};

pub mod awesome_event_model;
#[path = "db/schema.rs"]
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

fn insert_awesome_events_query(
    items_to_insert: Vec<AwesomeEvent>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use crate::schema::awesome_events::dsl::*;
    diesel::insert_into(awesome_events)
        .values(items_to_insert)
        .on_conflict((transaction_version, event_index))
        .do_nothing()
}

#[tokio::main]
async fn main() -> Result<()> {
    process(
        "karacurt_awesome_events_processor".to_string(),
        MIGRATIONS,
        async |transactions, conn_pool| {
            let events = transactions
                .par_iter()
                .map(|txn| {
                    let txn_version = txn.version as i64;
                    let block_height = txn.block_height as i64;
                    let txn_data = match txn.txn_data.as_ref() {
                        Some(data) => data,
                        None => {
                            warn!(
                                transaction_version = txn_version,
                                "Transaction data doesn't exist"
                            );
                            return vec![];
                        },
                    };
                    
                    let mut awesome_events = Vec::new();
                    let default = vec![];
                    let raw_events = match txn_data {
                        TxnData::User(tx_inner) => &tx_inner.events,
                        TxnData::BlockMetadata(tx_inner) => &tx_inner.events,
                        TxnData::Genesis(tx_inner) => &tx_inner.events,
                        _ => &default,
                    };

                    for (event_index, event) in raw_events.iter().enumerate() {
                        if AwesomeEvent::is_awesome_event(event) {
                            match AwesomeEvent::from_event(
                                event, 
                                txn_version, 
                                event_index as i64, 
                                block_height
                            ) {
                                Ok(awesome_event) => {
                                    info!(
                                        "Found AwesomeEvent: user={}, count={}, random={}, message='{}'",
                                        awesome_event.user_address,
                                        awesome_event.call_count,
                                        awesome_event.random_number,
                                        awesome_event.message
                                    );
                                    awesome_events.push(awesome_event);
                                }
                                Err(e) => {
                                    error!("Failed to parse AwesomeEvent: {}", e);
                                }
                            }
                        }
                    }
                    
                    awesome_events
                })
                .flatten()
                .collect::<Vec<AwesomeEvent>>();

            info!("Processing {} AwesomeEvents", events.len());
            
            // Store events in the database
            let execute_res = execute_in_chunks(
                conn_pool.clone(),
                insert_awesome_events_query,
                &events,
                MAX_DIESEL_PARAM_SIZE / AwesomeEvent::field_count(),
            ).await;
            
            match execute_res {
                Ok(_) => {
                    info!(
                        "AwesomeEvents version [{}, {}] stored successfully",
                        transactions.first().unwrap().version,
                        transactions.last().unwrap().version
                    );
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to store AwesomeEvents: {:?}", e);
                    Err(e)
                },
            }
        },
    ).await?;
    Ok(())
}
