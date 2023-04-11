use std::vec;

use bridge_hc_types::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn create_transaction(
    transaction: Transaction,
    preflight_responses: Vec<PreflightResponse>,
) -> ExternResult<ActionHashB64> {
    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(preflight_responses, vec![]).map_err(
                |countersigning_error| wasm_error!(countersigning_error.to_string()),
            )?,
        ),
        transaction.clone().try_into()?,
    );

    let transaction_action_hash = HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            EntryDefLocation::App(AppEntryDefLocation { zome_index: 0.into(), entry_def_index: 0.into() }), //TODO check/correct these indices
            EntryVisibility::Public,
            entry,
            ChainTopOrdering::Strict,
            // Countersigned entries MUST have strict ordering.
        ))
    })?;

    Ok(transaction_action_hash.into())
}
