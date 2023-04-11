use bridge_hc_types::Transaction;
use hdk::prelude::*;

mod handlers;
mod countersigning;
mod signals;
mod utils;

pub use handlers::*;
pub use utils::*;

#[derive(Serialize, Deserialize)]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Transaction(Transaction)
}

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions : BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
    functions.insert((zome_info()?.name.into(), FunctionName("recv_remote_signal".into())));

    let grant = CapGrantEntry {
        functions: GrantedFunctions::Listed(functions),
        access: CapAccess::Unrestricted,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions : BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
    functions.insert((zome_info()?.name.into(), FunctionName("transaction_preflight".into())));

    let grant = CapGrantEntry {
        functions: GrantedFunctions::Listed(functions),
        access: CapAccess::Unrestricted,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions : BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
    functions.insert((zome_info()?.name.into(), FunctionName("request_create_transaction".into())));

    let grant = CapGrantEntry {
        functions: GrantedFunctions::Listed(functions),
        access: CapAccess::Unrestricted,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    Ok(InitCallbackResult::Pass)
}
