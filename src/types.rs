use fvm_shared::ActorID;
use fvm_shared::clock::ChainEpoch;
use serde::{Deserialize, Serialize};

/// The libp2p peer id representation
pub type PeerID = String;
/// The libp2p multi address
pub type MultiAddr = String;

/// Member nodes information
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct NodeInfo {
    /// PeerID of the node
    id: PeerID,
    /// The creator of the node. Only creator can modifier other fields of this struct
    creator: ActorID,
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
}

impl NodeInfo {
    pub fn id(&self) -> &PeerID {
        &self.id
    }

    pub fn creator(&self) -> &ActorID {
        &self.creator
    }

    pub fn new(id: PeerID, creator: ActorID, addresses: Vec<MultiAddr>) -> Self {
        Self {
            id,
            creator,
            addresses,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Votes {
    /// Time of the last offline vote received by a
    /// checker.
    pub last_vote: ChainEpoch,
    /// Checkers that have voted
    pub votes: Vec<PeerID>,
}

impl Votes {
    pub fn new(epoch: ChainEpoch) -> Self {
        Self { last_vote: epoch, votes: vec![] }
    }

    pub fn has_voted(&self, p: &PeerID) -> bool {
        self.votes.contains(p)
    }

    pub fn within_threshold(&self, epoch: ChainEpoch, threshold: ChainEpoch) -> bool {
        self.last_vote + threshold < epoch
    }

    pub fn vote(&mut self, p: &PeerID) {
        self.votes.push(p.clone())
    }

    pub fn total_votes(&self) -> usize {
        self.votes.len()
    }
}

/// Constructor parameters
#[derive(Deserialize)]
pub struct InitParams {
    pub ids: Vec<String>,
    pub creators: Vec<ActorID>,
    pub addresses: Vec<Vec<String>>,
}
