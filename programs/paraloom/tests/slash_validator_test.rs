#[test]
fn slash_validator_forfeits_pending_rewards_proportionally() {
    // Arrange: active validator with pending rewards.
    // Execute: 50% slash.
    // Assert: pending_rewards is reduced by exactly 50%.
    // Execute: 100% slash.
    // Assert: pending_rewards is zero and cannot subsequently be claimed.
}

#[test]
fn slash_unbonding_validator_forfeits_pending_rewards() {
    // Arrange: inactive/unbonding validator with pending rewards.
    // Execute: slash.
    // Assert: unbonding stake and pending rewards are reduced by
    // the same percentage.
}
