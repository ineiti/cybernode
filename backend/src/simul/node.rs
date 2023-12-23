use byte_slice_cast::*;
use serde::Serialize;
use std::{fmt::Display, sync::mpsc::Sender};

use primitive_types::U256;
use ring::digest;
use tracing::{debug, error, info};

use super::{
    broker::{BMNode, BrokerMsg},
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
    pub from: U256,
    pub to: U256,
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

        Self::new(&Trusted::new(trusted::Config::default()))
    }

    /// Hash the secret to a public id.
    pub fn secret_to_id(secret: U256) -> U256 {
        digest::digest(&digest::SHA256, secret.as_byte_slice())
            .as_ref()
            .into()
    }

    pub fn id(&self) -> U256 {
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

    pub fn tick(&mut self, time: u64) -> Vec<NodeMsg> {
        if time > self.info.last_seen + 1_000_000 {
            info!("One second later");
        }
        self.info.last_seen = time;
        vec![]
    }

    pub fn action(&mut self, _task: BMNode) -> Vec<BrokerMsg> {
        todo!()
    }
}

#[derive(Clone, Serialize)]
pub struct NodeInfo {
    pub id: U256,
    pub name: String,
    pub last_seen: u64,
    pub mana: U256,
}

impl NodeInfo {
    pub fn random() -> Self {
        Self::with_id(rand::random::<[u8; 32]>().into())
    }

    pub fn with_id(id: U256) -> Self {
        Self {
            id,
            name: names::Generator::default().next().unwrap(),
            last_seen: 0,
            mana: U256::zero(),
        }
    }
}

impl Display for NodeInfo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:#018x}: '{}', mana={}, last_seen={}",
            self.id.as_ref()[0],
            self.name,
            self.mana,
            self.last_seen
        )
    }
}

impl std::fmt::Debug for NodeInfo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:#034x}: '{}', mana={}, last_seen={}",
            self.id, self.name, self.mana, self.last_seen
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
        assert_eq!(100u64, node1.info.last_seen);
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
