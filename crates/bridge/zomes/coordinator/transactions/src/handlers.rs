use std::collections::BTreeMap;

use bridge_hc_types::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{records_to_transactions, get_transactions_activity};

#[hdk_extern]
pub fn query_my_transactions(_: ()) -> ExternResult<BTreeMap<ActionHashB64, Transaction>> {
    let filter = ChainQueryFilter::new()
        .entry_type(transaction_entry_type()?)
        .include_entries(true);
    let records = query(filter)?;

    records_to_transactions(records)
}

#[hdk_extern]
pub fn get_transactions_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<BTreeMap<ActionHashB64, Transaction>> {
    let activity = get_transactions_activity(agent_pub_key.into())?;

    let get_inputs = activity
        .valid_activity
        .into_iter()
        .map(|(_, action_hash)| GetInput::new(action_hash.into(), GetOptions::default()))
        .collect();

    let maybe_records = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let records = maybe_records.into_iter().filter_map(|el| el).collect();

    let transactions = records_to_transactions(records)?;

    Ok(transactions)
}

#[hdk_extern]
pub fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<(ActionHashB64, Transaction)>> {
    let activity = get_transactions_activity(agent_pub_key)?;

    match activity.valid_activity.last() {
        None => Ok(None),
        Some((_seq, hash)) => {
            let record = get(hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
                "Couldn't get latest transaction",
            ))?;

            let entry = record
                .entry()
                .as_option()
                .ok_or(wasm_error!(String::from("Malformed transaction")))?;

            let transaction = Transaction::try_from_entry(entry.clone())?;

            let hash_b64 = ActionHashB64::from(hash.clone());

            Ok(Some((hash_b64, transaction)))
        }
    }
}

pub(crate) fn transaction_entry_type() -> ExternResult<EntryType> {
    let app_entry = AppEntryDef::new(0.into(), 0.into(), EntryVisibility::Public);
    Ok(EntryType::App(app_entry))
}
