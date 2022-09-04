use crate::Error;
use crate::traits::UptimeCheckerActor;
use crate::types::{InitParams, NodeInfo, PeerID};

pub struct Actor;

impl UptimeCheckerActor for Actor {
    fn init(params: InitParams) -> Result<(), Error> {
        todo!()
    }

    fn new_checker(params: NodeInfo) -> Result<(), Error> {
        todo!()
    }

    fn new_member(params: NodeInfo) -> Result<(), Error> {
        todo!()
    }

    fn edit_checker(params: NodeInfo) -> Result<(), Error> {
        todo!()
    }

    fn edit_member(params: NodeInfo) -> Result<(), Error> {
        todo!()
    }

    fn rm_checker(params: PeerID) -> Result<(), Error> {
        todo!()
    }

    fn rm_member(params: PeerID) -> Result<(), Error> {
        todo!()
    }

    fn report_checker(params: PeerID) -> Result<(), Error> {
        todo!()
    }
}