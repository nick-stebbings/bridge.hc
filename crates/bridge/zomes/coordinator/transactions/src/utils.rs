use std::collections::BTreeMap;

use bridge_hc_types::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::transaction_entry_type;

pub fn records_to_transactions(
    records: Vec<Record>,
) -> ExternResult<BTreeMap<ActionHashB64, Transaction>> {
    let transactions = records
        .into_iter()
        .map(|record| {
            let entry = record
                .entry()
                .as_option()
                .ok_or(wasm_error!(String::from("Malformed transaction")))?;

            let transaction = Transaction::try_from_entry(entry.clone())?;

            let hash_b64 = ActionHashB64::from(record.action_address().clone());

            Ok((hash_b64, transaction))
        })
        .collect::<ExternResult<BTreeMap<ActionHashB64, Transaction>>>()?;

    Ok(transactions)
}

pub fn get_transactions_activity(agent_pub_key: AgentPubKeyB64) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(transaction_entry_type()?);

    let activity = get_agent_activity(agent_pub_key.into(), filter, ActivityRequest::Full)?;

    Ok(activity)
}
