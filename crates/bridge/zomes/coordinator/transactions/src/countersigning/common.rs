use transactions_and_requests_integrity::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn create_transaction(
    transaction: Transaction,
    preflight_responses: Vec<PreflightResponse>,
) -> ExternResult<ActionHashB64> {
    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(preflight_responses).map_err(
                |countersigning_error| wasm_error!(countersigning_error.to_string()),
            )?,
        ),
        transaction.clone().try_into()?,
    );

    let transaction_action_hash = HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            Transaction::entry_def_id(),
            entry,
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })?;

    Ok(transaction_action_hash.into())
}
