use crate::blockstore::{make_empty_map, Blockstore};
use crate::types::{NodeInfo, PeerID};
use crate::Error;
use cid::Cid;
use fvm_ipld_encoding::{to_vec, CborStore, DAG_CBOR};
use fvm_ipld_hamt::BytesKey;
use multihash::Code;
use serde::{Deserialize, Serialize};

/// The state object.
#[derive(Debug, Serialize, Deserialize)]
pub struct HamtState {
    // /// The list of node members in the registry
    // members: HashMap<PeerID, NodeInfo>,
    // /// List of checkers registered in the system.
    // checkers: HashMap<PeerID, NodeInfo>,
    // /// Data structure used to signal offline checkers.
    // offline_checkers: HashMap<PeerID, NodeInfo>,

    /// The list of node members in the registry
    members: Cid,
    /// List of checkers registered in the system.
    checkers: Cid,
    /// Data structure used to signal offline checkers.
    offline_checkers: Cid,
}

/// We should probably have a derive macro to mark an object as a state object,
/// and have load and save methods automatically generated for them as part of a
/// StateObject trait (i.e. impl StateObject for State).
impl HamtState {
    pub fn new(nodes: Vec<NodeInfo>) -> Result<Self, Error> {
        let mut checker_map = make_empty_map::<_, NodeInfo>(&Blockstore);
        for n in nodes {
            checker_map.set(BytesKey::from(n.id().as_bytes()), n)?;
        }
        Ok(HamtState {
            members: make_empty_map::<_, NodeInfo>(&Blockstore).flush()?,
            checkers: checker_map.flush()?,
            offline_checkers: make_empty_map::<_, PeerID>(&Blockstore).flush()?,
        })
    }

    pub fn load() -> Result<Self, Error> {
        let root = fvm_sdk::sself::root()?;
        (Blockstore.get_cbor::<Self>(&root)?).ok_or(Error::CannotDeserialize)
    }

    pub fn save(&self) -> Result<Cid, Error> {
        let serialized = to_vec(self)?;
        let cid = fvm_sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())?;
        fvm_sdk::sself::set_root(&cid)?;
        Ok(cid)
    }
}
