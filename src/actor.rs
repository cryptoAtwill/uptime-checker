use std::marker::PhantomData;
use crate::traits::{LoadableState, UptimeCheckerActor};
use crate::types::{InitParams, MultiAddr, NodeInfo, NodeInfoPayload, PeerID, ReportPayload};
use crate::{ensure, Error};

const THRESHOLD_NUM: usize = 20000;
const THRESHOLD_DUM: usize = 30000;

pub struct Actor<S: LoadableState> {
    _phantom: PhantomData<S>
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
                    .map(MultiAddr::from)
                    .collect(),
            ));
        }
        let state = S::new(nodes)?;
        state.save()?;
        Ok(())
    }

    fn new_checker(c: NodeInfoPayload) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_checker(NodeInfo::from(c))?;
        s.save()?;
        Ok(())
    }

    fn new_member(m: NodeInfoPayload) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_node(NodeInfo::from(m))?;
        s.save()?;
        Ok(())
    }

    fn edit_checker(c: NodeInfoPayload) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_checker(NodeInfo::from(c))?;
        s.save()?;
        Ok(())
    }

    fn edit_member(m: NodeInfoPayload) -> Result<(), Error> {
        let mut s = S::load()?;
        s.upsert_node(NodeInfo::from(m))?;
        s.save()?;
        Ok(())
    }

    fn rm_checker() -> Result<(), Error> {
        let mut s = S::load()?;
        s.remove_checker(&fvm_sdk::message::caller())?;
        s.save()?;
        Ok(())
    }

    fn rm_member() -> Result<(), Error> {
        let mut s = S::load()?;
        s.remove_node(&fvm_sdk::message::caller())?;
        s.save()?;
        Ok(())
    }

    fn report_checker(p: ReportPayload) -> Result<(), Error> {
        let mut s = S::load()?;
        let caller = fvm_sdk::message::caller();

        ensure!(s.is_checker(&caller)?, Error::NotCaller)?;
        ensure!(!s.has_voted(&p.checker, &caller)?, Error::AlreadyVoted)?;

        let votes = s.record_voted(&p.checker, &caller)?;

        // perform checks
        let total_checkers = s.total_checkers();
        if total_checkers * THRESHOLD_NUM / THRESHOLD_DUM < votes {
            s.remove_checker_unchecked(&p.checker)?;
        }

        Ok(())
    }
}
