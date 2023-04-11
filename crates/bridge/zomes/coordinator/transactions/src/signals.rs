use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use bridge_hc_types::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    NewTransactionCreated {
        transaction_hash: ActionHashB64,
        transaction: Transaction,
    },
}
