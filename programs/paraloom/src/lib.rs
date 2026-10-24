let old_rewards = validator_account.pending_rewards;
let rewards_slash = (old_rewards as u128 * slash_percentage as u128 / 100) as u64;

validator_account.pending_rewards = old_rewards.saturating_sub(rewards_slash);

if rewards_slash > 0 {
    msg!("Forfeited pending rewards: {} lamports", rewards_slash);
}
