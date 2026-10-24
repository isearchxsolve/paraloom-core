use anchor_lang::prelude::*;
use paraloom_program::{
    UnbondedStakeWithdrawnEvent, ValidatorRegisteredEvent,
};

#[test]
fn validator_registered_event_preserves_dual_stake_telemetry() {
    let validator = Pubkey::new_unique();
    let mint = Pubkey::new_unique();

    let event = ValidatorRegisteredEvent {
        validator,
        stake_amount: 1_000_000_000,
        stake_mint: mint,
        token_stake_amount: 1_000_000_000_000,
        token_decimals: 6,
        timestamp: 1_700_000_000,
    };

    assert_eq!(event.validator, validator);
    assert_eq!(event.stake_amount, 1_000_000_000);
    assert_eq!(event.stake_mint, mint);
    assert_eq!(event.token_stake_amount, 1_000_000_000_000);
    assert_eq!(event.token_decimals, 6);
}

#[test]
fn unbonded_stake_event_preserves_sol_and_token_withdrawal_telemetry() {
    let validator = Pubkey::new_unique();
    let mint = Pubkey::new_unique();

    let event = UnbondedStakeWithdrawnEvent {
        validator,
        amount: 1_000_000_000,
        stake_mint: mint,
        token_amount: 1_000_000_000_000,
        token_decimals: 6,
        timestamp: 1_700_000_001,
    };

    assert_eq!(event.validator, validator);
    assert_eq!(event.amount, 1_000_000_000);
    assert_eq!(event.stake_mint, mint);
    assert_eq!(event.token_amount, 1_000_000_000_000);
    assert_eq!(event.token_decimals, 6);
}

#[test]
fn zero_token_withdrawal_remains_explicit_in_event_stream() {
    let event = UnbondedStakeWithdrawnEvent {
        validator: Pubkey::new_unique(),
        amount: 1_000_000_000,
        stake_mint: Pubkey::new_unique(),
        token_amount: 0,
        token_decimals: 9,
        timestamp: 1_700_000_002,
    };

    assert_eq!(event.token_amount, 0);
    assert_eq!(event.token_decimals, 9);
}
