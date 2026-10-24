#[cfg(test)]
mod tests {
    use super::peer_id_from_multiaddr;
    use libp2p::{identity, Multiaddr, PeerId};

    fn fresh_peer_id() -> PeerId {
        PeerId::from(identity::Keypair::generate_ed25519().public())
    }

    #[test]
    fn extracts_peer_id_from_direct_terminal_p2p_address() {
        let target_peer = fresh_peer_id();
        let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/4001/p2p/{target_peer}")
            .parse()
            .expect("direct peer multiaddr must parse");

        assert_eq!(peer_id_from_multiaddr(&addr), Some(target_peer));
    }

    #[test]
    fn extracts_trailing_target_peer_id_from_relay_circuit_address() {
        let relay_peer = fresh_peer_id();
        let target_peer = fresh_peer_id();

        let addr: Multiaddr = format!(
            "/ip4/127.0.0.1/tcp/4001/p2p/{relay_peer}/p2p-circuit/p2p/{target_peer}"
        )
        .parse()
        .expect("relay circuit target multiaddr must parse");

        assert_eq!(
            peer_id_from_multiaddr(&addr),
            Some(target_peer),
            "the trailing target peer must win over the relay peer"
        );
    }

    #[test]
    fn rejects_bare_relay_circuit_address_without_target_peer_id() {
        let relay_peer = fresh_peer_id();

        let addr: Multiaddr = format!(
            "/ip4/127.0.0.1/tcp/4001/p2p/{relay_peer}/p2p-circuit"
        )
        .parse()
        .expect("bare relay circuit multiaddr must parse");

        assert_eq!(
            peer_id_from_multiaddr(&addr),
            None,
            "the relay peer is not the target peer for a bare circuit address"
        );
    }

    #[test]
    fn rejects_transport_address_without_terminal_peer_id() {
        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/4001"
            .parse()
            .expect("transport multiaddr must parse");

        assert_eq!(peer_id_from_multiaddr(&addr), None);
    }

    #[test]
    fn uses_last_peer_id_when_multiple_peer_id_components_are_present() {
        let first_peer = fresh_peer_id();
        let trailing_peer = fresh_peer_id();

        let addr: Multiaddr = format!(
            "/ip4/127.0.0.1/tcp/4001/p2p/{first_peer}/p2p-circuit/p2p/{trailing_peer}"
        )
        .parse()
        .expect("multi-peer multiaddr must parse");

        assert_ne!(first_peer, trailing_peer);
        assert_eq!(peer_id_from_multiaddr(&addr), Some(trailing_peer));
    }
}
