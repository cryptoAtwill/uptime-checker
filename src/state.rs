use cid::Cid;
use crate::Error;
use crate::types::InitParams;

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct State {
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
impl State {
    pub fn new(params: InitParams) -> Result<Self, Error> {
        let empty_members_map = make_empty_map::<_, ()>(&Blockstore).flush()?;
        let checkers_map = make_empty_map::<_, ()>(&Blockstore).flush()?;
        let empty_offline_map = make_empty_map::<_, ()>(&Blockstore).flush()?;

        Ok(State {
            members: empty_members_map,
            checkers: checkers_map,
            offline_checkers: empty_offline_map,
        })
    }

    pub fn load() -> Result<Self, Error> {
        // First, load the current state root.
        let root = fvm_sdk::sself::root()?;
        Blockstore.get_cbor::<Self>(&root)
    }

    // pub fn save(&self) -> Cid {
    //     let serialized = match to_vec(self) {
    //         Ok(s) => s,
    //         Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
    //     };
    //     let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
    //     {
    //         Ok(cid) => cid,
    //         Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
    //     };
    //     if let Err(err) = sdk::sself::set_root(&cid) {
    //         abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
    //     }
    //     cid
    // }
}