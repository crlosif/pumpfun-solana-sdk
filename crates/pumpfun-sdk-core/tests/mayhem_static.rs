use pumpfun_sdk_core::{ids, pda};

#[test]
fn mayhem_static_accounts_match_derived_pdas() {
    // These are published in Pump docs. If this breaks, it likely means:
    // - wrong program id
    // - program changed
    // - you are on a different cluster
    let (gp, _) = pda::mayhem_global_params(&ids::DEFAULT_MAYHEM_PROGRAM_ID);
    assert_eq!(gp, ids::MAYHEM_GLOBAL_PARAMS);

    let (vault, _) = pda::mayhem_sol_vault(&ids::DEFAULT_MAYHEM_PROGRAM_ID);
    assert_eq!(vault, ids::MAYHEM_SOL_VAULT);
}
