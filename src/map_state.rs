use std::collections::HashMap;
use crate::blockstore::{Blockstore};
use crate::types::{NodeInfo, PeerID, Votes};
use crate::Error;
use cid::Cid;
use fvm_ipld_encoding::{to_vec, CborStore, DAG_CBOR};
use fvm_shared::clock::ChainEpoch;
use multihash::Code;
use serde::{Deserialize, Serialize};
use crate::traits::LoadableState;

/// The state object.
#[derive(Debug, Serialize, Deserialize)]
pub struct MapState {
    /// The list of node members in the registry
    members: HashMap<PeerID, NodeInfo>,
    /// List of checkers registered in the system.
    checkers: HashMap<PeerID, NodeInfo>,
    /// Data structure used to signal offline checkers.
    offline_checkers: HashMap<PeerID, Votes>,
}

/// We should probably have a derive macro to mark an object as a state object,
/// and have load and save methods automatically generated for them as part of a
/// StateObject trait (i.e. impl StateObject for State).
impl LoadableState for MapState {
    fn new(nodes: Vec<NodeInfo>) -> Result<Self, Error> {
        let mut checker_map = HashMap::new();
        for n in nodes {
            checker_map.insert(n.id().clone(), n);
        }
        Ok(MapState {
            members: Default::default(),
            checkers: checker_map,
            offline_checkers: Default::default(),
        })
    }

    fn upsert_node(&mut self, node: NodeInfo) -> Result<(), Error> {
        Self::upsert(&mut self.members, node)
    }

    fn remove_node(&mut self, id: &PeerID) -> Result<(), Error> {
        Self::remove(&mut self.members, id)
    }

    fn upsert_checker(&mut self, node: NodeInfo) -> Result<(), Error> {
        Self::upsert(&mut self.checkers, node)
    }

    fn remove_checker(&mut self, id: &PeerID) -> Result<(), Error> {
        Self::remove(&mut self.checkers, id)
    }

    fn remove_checker_unchecked(&mut self, id: &PeerID) -> Result<(), Error> {
        self.checkers.remove(id);
        Ok(())
    }

    fn has_voted(&self, p: &PeerID) -> bool {
        if !self.offline_checkers.contains_key(p) {
            return false;
        }
        let vote = self.offline_checkers.get(p).unwrap();
        vote.has_voted(p)
    }

    fn record_voted(&mut self, p: &PeerID) -> usize {
        if !self.offline_checkers.contains_key(p) {
            let mut vote = Votes::new(fvm_sdk::network::curr_epoch());
            vote.vote(p);
            self.offline_checkers.insert(p.clone(), vote);
            return 1;
        }

        let t = self.vote_threshold();
        let vote = self.offline_checkers.get_mut(p).unwrap();
        if vote.within_threshold(fvm_sdk::network::curr_epoch(), t) {
            vote.vote(p);
            vote.total_votes()
        } else {
            self.offline_checkers.remove(p);
            0
        }
    }

    fn total_checkers(&self) -> usize {
        self.checkers.len()
    }

    fn vote_threshold(&self) -> ChainEpoch {
        200
    }

    fn load() -> Result<Self, Error> {
        let root = fvm_sdk::sself::root()?;
        (Blockstore.get_cbor::<Self>(&root)?).ok_or(Error::CannotDeserialize)
    }

    fn save(&self) -> Result<Cid, Error> {
        let serialized = to_vec(self)?;
        let cid = fvm_sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())?;
        fvm_sdk::sself::set_root(&cid)?;
        Ok(cid)
    }
}

impl MapState {
    fn ensure_owner(b: &NodeInfo) -> Result<(), Error> {
        if fvm_sdk::message::caller() != *b.creator() {
            Err(Error::NotOwner)
        } else {
            Ok(())
        }
    }

    fn upsert(map: &mut HashMap<PeerID, NodeInfo>, node: NodeInfo) -> Result<(), Error> {
        if map.contains_key(node.id()) {
            let n = map.get(node.id()).unwrap();
            Self::ensure_owner(&n)?;
            // if n.creator() and node.creator() are not the same, it is ok as transfer ownership
        }
        map.insert(node.id().clone(), node);
        Ok(())
    }

    fn remove(map: &mut HashMap<PeerID, NodeInfo>, id: &PeerID) -> Result<(), Error> {
        if map.contains_key(id) {
            let n = map.get(id).unwrap();
            Self::ensure_owner(&n)?;
            map.remove(id);
            Ok(())
        } else {
            Err(Error::NotExists)
        }
    }
}
