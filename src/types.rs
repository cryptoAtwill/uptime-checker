use fvm_shared::address::{Address};
use fvm_shared::clock::ChainEpoch;
use serde::Deserialize;

/// The libp2p peer id representation
pub type PeerID = String;
/// The libp2p multi address
pub type MultiAddr = String;

/// Member nodes information
#[derive(Deserialize)]
pub struct NodeInfo {
    /// PeerID of the node
    id: PeerID,
    /// The creator of the node. Only creator can modifier other fields of this struct
    creator: Address,
    /// List of multiaddresses exposed by the node
    /// along with the supported healthcheck endpoints.
    ///
    /// e.g. [ /ip4/10.1.1.1/quic/8080/p2p/<peer_id>/ping,
    ///        /ip4/10.1.1.1/tcp/8081/http/get/healtcheck,
    ///      ]
    /// These multiaddresses are signalling that the liveliness
    /// can be checked by using the default libp2p ping protocol
    /// in the first multiaddress, or by sending a GET HTTP
    /// query to the /healtchek endpoint at 10.1.1.1:8081.
    addresses: Vec<MultiAddr>,
    /// The updated epoch time
    updated: ChainEpoch,
}

pub struct Votes {
    /// Time of the last offline vote received by a
    /// checker.
    pub last_vote: ChainEpoch,
    /// Checkers that have voted
    pub votes: Vec<PeerID>,
}

/// Constructor parameters
#[derive(Deserialize)]
pub struct InitParams {
    /// Initial checkers to populate the uptimeActor state
    pub initial_checkers: Vec<NodeInfo>,
}