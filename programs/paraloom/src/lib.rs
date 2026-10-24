pub fn withdraw_unbonded_stake(ctx: Context<WithdrawUnbondedStake>) -> Result<()> {
    let validator_account = &mut ctx.accounts.validator_account;
    let amount = validator_account.unbonding_amount;
    let token_amount = validator_account.token_unbonding_amount;
    let validator = validator_account.validator;
    let stake_mint = ctx.accounts.stake_mint.key();
    let token_decimals = ctx.accounts.stake_mint.decimals;

    let zero_amount_exit_close = amount == 0
        && !validator_account.is_active
        && validator_account.stake_amount == 0
        && validator_account.unbonding_slot > 0;

    require!(
        amount > 0 || zero_amount_exit_close,
        BridgeError::NothingUnbonding
    );
    require!(
        Clock::get()?.slot >= validator_account.unbonding_slot,
        BridgeError::UnbondingNotElapsed
    );
    require!(
        validator_account.pending_rewards == 0,
        BridgeError::PendingRewardsUnclaimed
    );

    // Complete every external CPI before committing local lamport deltas.
    if token_amount > 0 {
        let auth_bump = ctx.bumps.stake_vault_authority;
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"stake_vault_authority", &[auth_bump]]];

        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.stake_token_vault.to_account_info(),
                    mint: ctx.accounts.stake_mint.to_account_info(),
                    to: ctx.accounts.validator_token_account.to_account_info(),
                    authority: ctx.accounts.stake_vault_authority.to_account_info(),
                },
                signer_seeds,
            ),
            token_amount,
            token_decimals,
        )?;
    }

    // The validator PDA is owned by this program, therefore its lamports are
    // transferred directly by the owner program. Checked arithmetic prevents
    // malformed state from wrapping either balance.
    let validator_pda_info = validator_account.to_account_info();
    let validator_wallet_info = ctx.accounts.validator.to_account_info();

    let pda_lamports = validator_pda_info.lamports();
    let wallet_lamports = validator_wallet_info.lamports();

    let updated_pda_lamports = pda_lamports
        .checked_sub(amount)
        .ok_or(BridgeError::InvalidAmount)?;
    let updated_wallet_lamports = wallet_lamports
        .checked_add(amount)
        .ok_or(BridgeError::InvalidAmount)?;

    **validator_pda_info.try_borrow_mut_lamports()? = updated_pda_lamports;
    **validator_wallet_info.try_borrow_mut_lamports()? = updated_wallet_lamports;

    validator_account.unbonding_amount = 0;
    validator_account.token_unbonding_amount = 0;

    emit!(UnbondedStakeWithdrawnEvent {
        validator,
        amount,
        stake_mint,
        token_amount,
        token_decimals,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "Unbonded dual stake withdrawn: validator={}, lamports={}, mint={}, tokens={}",
        validator,
        amount,
        stake_mint,
        token_amount
    );
    Ok(())
}
