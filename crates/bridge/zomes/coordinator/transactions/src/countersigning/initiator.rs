use bridge_hc_types::{AttemptCreateTransactionInput, Transaction};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::transaction_entry_type;

use super::common::create_transaction;
use super::responder::TransactionPreflight;

#[hdk_extern]
pub fn attempt_create_transaction(
    input: AttemptCreateTransactionInput,
) -> ExternResult<(ActionHashB64, Transaction)> {
    let counterparty = input.transaction.get_counterparty()?.agent_pub_key;

    let my_pub_key = agent_info()?.agent_initial_pubkey;

    let countersigning_agents = vec![
        (AgentPubKey::from(my_pub_key.clone()), vec![]),
        (AgentPubKey::from(counterparty.clone()), vec![]),
    ];
    let preflight_request =
        build_preflight_request(input.transaction.clone(), countersigning_agents)?;

    let response = call_remote(
        counterparty.clone().into(),
        zome_info()?.name,
        "transaction_preflight".into(),
        None,
        TransactionPreflight {
            preflight_request: preflight_request.clone(),
            chain_top: input.counterparty_chain_top.clone(),
        },
    )?;

    let result = match response.clone() {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(wasm_error!(format!(
            "Error with fn transaction_preflight: {:?}",
            response
        ))),
    }?;

    let counterparty_response: PreflightResponse = result.decode().map_err(|e| wasm_error!(e))?;

    let my_response = match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(wasm_error!(String::from(
            "Couldn't lock our own chain",
        ))),
    }?;

    let response = call_remote(
        counterparty.clone().into(),
        zome_info()?.name,
        "request_create_transaction".into(),
        None,
        vec![my_response.clone(), counterparty_response.clone()],
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(wasm_error!(format!(
            "Error with fn request_create_transaction {:?}",
            response
        ))),
    }?;

    let _counterparty_action_hash: ActionHashB64 = result.decode().map_err(|e| wasm_error!(e))?;

    let action_hash = create_transaction(
        input.transaction.clone(),
        vec![my_response, counterparty_response],
    )?;

    Ok((action_hash.into(), input.transaction))
}

fn build_preflight_request(
    transaction: Transaction,
    countersigning_agents: CounterSigningAgents,
) -> ExternResult<PreflightRequest> {
    let transaction_hash = hash_entry(&transaction)?;

    let times = session_times_from_millis(5_000)?;

    let action_base = ActionBase::Create(CreateBase::new(transaction_entry_type()?));

    let bytes = SerializedBytes::try_from(transaction.clone()).map_err(|e| wasm_error!(e))?;

    let preflight_bytes = PreflightBytes(bytes.bytes().to_vec());

    let preflight_request = PreflightRequest::try_new(
        transaction_hash,
        countersigning_agents,
        vec![],
        0,
        false,
        times,
        action_base,
        preflight_bytes
    )
    .map_err(|err| wasm_error!(format!("Could not create preflight request: {:?}", err)))?;

    Ok(preflight_request)
}
