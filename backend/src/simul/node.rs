use serde::Serialize;
use std::{fmt::Display, sync::mpsc::Sender};

use tracing::{debug, error, info};

use super::{
    broker::{BMNode, BrokerMsg},
    node_types::{Mana, NodeID},
    trusted::{TReqMsg, TrustedRequest},
};

/// Node is a simulation which can answer to certain messages.
/// If it receives regular 'tick's, it will send out some messages on its own.
#[derive(Debug)]
pub struct Node {
    info: NodeInfo,
    trusted: Sender<TrustedRequest>,
}

#[derive(Debug)]
pub struct NodeMsg {
    pub from: NodeID,
    pub to: NodeID,
    pub msg: Msg,
}

#[derive(Debug)]
pub enum Msg {
    Ping,
    Pong,
}

impl Node {
    pub fn new(trusted: &Sender<TrustedRequest>) -> Self {
        Self::from_info(NodeInfo::random(), trusted)
    }

    /// Make a dummy node with a dummy channel to Trusted.
    #[cfg(test)]
    pub fn dummy() -> Self {
        use super::trusted;
        use crate::simul::trusted::Trusted;

        Self::new(&Trusted::new(trusted::Config::default(), 0))
    }

    pub fn id(&self) -> NodeID {
        self.info.id
    }

    pub fn info(&self) -> NodeInfo {
        self.info.clone()
    }

    fn update_trusted(&self) {
        if let Err(e) = TReqMsg::Register(self.info.clone()).send(&self.trusted) {
            error!("While registering node: {e:?}");
        }
    }

    pub fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg> {
        let mut out = vec![];
        debug!("Processing message {input:?}");
        match &input.msg {
            Msg::Ping => out.push(NodeMsg {
                from: self.id(),
                to: input.from,
                msg: Msg::Pong,
            }),
            Msg::Pong => info!("Got pong {input:?}"),
        }
        out
    }

    pub fn from_info(info: NodeInfo, trusted: &Sender<TrustedRequest>) -> Self {
        let reply = Self {
            info,
            trusted: trusted.clone(),
        };
        reply.update_trusted();
        reply
    }

    pub fn tick(&mut self, _time: u128) -> Vec<NodeMsg> {
        vec![]
    }

    pub fn action(&mut self, _task: BMNode) -> Vec<BrokerMsg> {
        todo!()
    }
}

#[derive(Clone, Serialize)]
pub struct NodeInfo {
    pub id: NodeID,
    pub name: String,
    pub mana: Mana,
}

impl NodeInfo {
    pub fn random() -> Self {
        Self::with_id(NodeID::random())
    }

    pub fn with_id(id: NodeID) -> Self {
        Self {
            id,
            name: names::Generator::default().next().unwrap(),
            mana: Mana::zero(),
        }
    }
}

impl Display for NodeInfo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}: '{}', mana={}",
            self.id, self.name, self.mana,
        )
    }
}

impl std::fmt::Debug for NodeInfo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:?}: '{}', mana={}",
            self.id, self.name, self.mana
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_receive() {
        let mut node1 = Node::dummy();
        let mut node2 = Node::dummy();
        node1.tick(100);
        let mut msgs = node1.receive(NodeMsg {
            from: node2.id(),
            to: node1.id(),
            msg: Msg::Ping,
        });
        assert_eq!(1, msgs.len());
        msgs = node2.receive(msgs.remove(0));
        assert_eq!(0, msgs.len());
    }
}
