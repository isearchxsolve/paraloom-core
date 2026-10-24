#[cfg(test)]
mod cosign_count_tests {
    use super::{
        cosign_nullifier_keys, reserve_cosign_counts, MAX_COSIGN_COUNTS,
        MAX_COSIGNS_PER_SETTLEMENT,
    };
    use std::collections::HashMap;

    #[test]
    fn authority_rotation_cannot_create_distinct_nullifier_budget_keys() {
        let first = cosign_nullifier_keys(&[[0x11; 32], [0x22; 32]]);
        let second = cosign_nullifier_keys(&[[0x11; 32], [0x22; 32]]);

        assert_eq!(first, second);
        assert_eq!(first.len(), 2);
        assert_eq!(first[0], hex::encode([0x11; 32]));
        assert_eq!(first[1], hex::encode([0x22; 32]));
        assert!(first.iter().all(|key| !key.contains(':')));
    }

    #[test]
    fn fifth_cosign_for_same_nullifier_is_rejected() {
        let keys = cosign_nullifier_keys(&[[0x41; 32], [0x42; 32]]);
        let mut counts = HashMap::new();

        for _ in 0..MAX_COSIGNS_PER_SETTLEMENT {
            reserve_cosign_counts(&mut counts, &keys).unwrap();
        }

        let before = counts.clone();
        let error = reserve_cosign_counts(&mut counts, &keys).unwrap_err();

        assert_eq!(error, "co-sign count limit reached for an input nullifier");
        assert_eq!(counts, before);
        assert_eq!(
            counts.get(&hex::encode([0x41; 32])),
            Some(&MAX_COSIGNS_PER_SETTLEMENT)
        );
        assert_eq!(
            counts.get(&hex::encode([0x42; 32])),
            Some(&MAX_COSIGNS_PER_SETTLEMENT)
        );
    }

    #[test]
    fn full_cache_rejects_new_key_without_evicting_live_budget() {
        let protected = hex::encode([0xaa; 32]);
        let mut counts = HashMap::new();

        counts.insert(protected.clone(), MAX_COSIGNS_PER_SETTLEMENT);

        for index in 0..(MAX_COSIGN_COUNTS - 1) {
            counts.insert(format!("filler-{index:04x}"), 1);
        }

        assert_eq!(counts.len(), MAX_COSIGN_COUNTS);

        let fresh = vec![hex::encode([0xbb; 32])];
        let before = counts.clone();
        let error = reserve_cosign_counts(&mut counts, &fresh).unwrap_err();

        assert_eq!(error, "co-sign count cache is full");
        assert_eq!(counts, before);
        assert_eq!(
            counts.get(&protected),
            Some(&MAX_COSIGNS_PER_SETTLEMENT)
        );
        assert!(!counts.contains_key(&hex::encode([0xbb; 32])));
    }

    #[test]
    fn duplicate_nullifier_is_counted_once_per_settlement() {
        let duplicated = cosign_nullifier_keys(&[[0x77; 32], [0x77; 32]]);
        let mut counts = HashMap::new();

        reserve_cosign_counts(&mut counts, &duplicated).unwrap();

        assert_eq!(duplicated.len(), 1);
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get(&hex::encode([0x77; 32])), Some(&1));
    }

    #[test]
    fn reservation_is_atomic_when_one_input_has_exhausted_its_budget() {
        let first = hex::encode([0x01; 32]);
        let second = hex::encode([0x02; 32]);
        let mut counts = HashMap::new();

        counts.insert(first.clone(), MAX_COSIGNS_PER_SETTLEMENT);
        counts.insert(second.clone(), 2);

        let before = counts.clone();
        let error = reserve_cosign_counts(&mut counts, &[first, second]).unwrap_err();

        assert_eq!(error, "co-sign count limit reached for an input nullifier");
        assert_eq!(counts, before);
    }
}
