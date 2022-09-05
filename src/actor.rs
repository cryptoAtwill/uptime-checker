use crate::traits::{LoadableState, UptimeCheckerActor};
use crate::types::{InitParams, MultiAddr, NodeInfo, PeerID};
use crate::Error;

const THRESHOLD_NUM: usize = 20000;
const THRESHOLD_DUM: usize = 30000;

pub struct Actor<S: LoadableState> {
    state: S
}

impl <S: LoadableState> UptimeCheckerActor for Actor<S> {
    fn init(params: InitParams) -> Result<(), Error> {
        // TODO: perform simple checks
        let len = params.ids.len();
        let mut nodes = vec![];
        for i in 0..len {
            nodes.push(NodeInfo::new(
                PeerID::from(&params.ids[i]),
                params.creators[i],
                params.addresses[i]
                    .iter()
                    .map(|e| MultiAddr::from(e))
                    .collect(),
            ));
        }
        let state = S::new(nodes)?;
        state.save()?;
        Ok(())
    }

    fn new_checker(c: NodeInfo) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_checker(c)?;
        s.save()?;
        Ok(())
    }

    fn new_member(m: NodeInfo) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_node(m)?;
        s.save()?;
        Ok(())
    }

    fn edit_checker(n: NodeInfo) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_checker(n)?;
        s.save()?;
        Ok(())
    }

    fn edit_member(m: NodeInfo) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_node(m)?;
        s.save()?;
        Ok(())
    }

    fn rm_checker(p: PeerID) -> Result<(), Error> {
        let mut s = S::load()?;
        s.remove_checker(&p)?;
        s.save()?;
        Ok(())
    }

    fn rm_member(p: PeerID) -> Result<(), Error> {
        let mut s = S::load()?;
        s.remove_node(&p)?;
        s.save()?;
        Ok(())
    }

    fn report_checker(p: PeerID) -> Result<(), Error> {
        let mut s = S::load()?;

        if s.has_voted(&p) {
            return Err(Error::AlreadyVoted);
        }

        let votes = s.record_voted(&p);

        // perform checks
        let total_checkers = s.total_checkers();
        if total_checkers * THRESHOLD_NUM / THRESHOLD_DUM < votes {
            s.remove_checker_unchecked(&p)?;
        }

        Ok(())
    }
}
