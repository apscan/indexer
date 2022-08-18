use crate::schema::blocks;
use serde::{Deserialize, Serialize};
use aptos_rest_client::aptos_api_types::Event as APIEvent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewBlockEventAPI {
    pub epoch: String,
    pub round: String,
    pub height: String,
    pub time_microseconds: String,
    pub previous_block_votes: serde_json::Value,
    pub failed_proposer_indices: serde_json::Value,
}


#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = "blocks")]
#[primary_key(height)]
pub struct Block {
    pub transaction_version: i64,
    pub epoch: i64,
    pub round: i64,
    pub height: i64,
    pub hash: String,
    pub time_microseconds: i64,
    pub previous_block_votes: serde_json::Value,
    pub failed_proposer_indices: serde_json::Value,
}

impl Block {
    pub fn from_event(transaction_version: i64, block_hash : String, event: &APIEvent) -> Option<Self> {
        let data = event.data.clone();
        match event.typ.to_string().as_str() {
            "0x1::block::NewBlockEvent" => {
                let block_event = serde_json::from_value::<NewBlockEventAPI>(data).unwrap();
                Some(Block{
                    transaction_version,
                    epoch: block_event.epoch.parse::<i64>().unwrap(),
                    round: block_event.round.parse::<i64>().unwrap(),
                    height: block_event.height.parse::<i64>().unwrap(),
                    hash: block_hash,
                    time_microseconds: block_event.time_microseconds.parse::<i64>().unwrap(),
                    previous_block_votes: block_event.previous_block_votes,
                    failed_proposer_indices: block_event.failed_proposer_indices
                })
            }
            _ => {None}
        
    }
}

    pub fn from_events(transaction_version: i64, block_hash: String, events: &[APIEvent]) -> Vec<Self> {
            events
                .iter()
                .filter_map(|event| Self::from_event(transaction_version, block_hash.clone(), event))
                .collect()
    }
}