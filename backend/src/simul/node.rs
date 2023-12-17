use std::sync::mpsc::Sender;
use byte_slice_cast::*;


use primitive_types::U256;
use ring::digest;
use tracing::{debug, info};

use super::{
    broker::{BrokerAction, Module},
    trusted::TrustedRequest,
};

/// Node is a simulation which can answer to certain messages.
/// If it receives regular 'tick's, it will send out some messages on its own.
#[derive(Debug)]
pub struct Node {
    pub id: U256,
    pub mana: U256,
    last: u64,
    online: bool,
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
    pub fn new(trusted: &Sender<TrustedRequest>, online: bool) -> Self {
        Self {
            id: rand::random::<[u8; 32]>().into(),
            mana: 0.into(),
            last: 0,
            online,
            trusted: trusted.clone(),
        }
    }

    /// Make a dummy node with a dummy channel to Trusted.
    #[cfg(test)]
    pub fn dummy(online: bool) -> Self {
        use crate::simul::trusted::Trusted;
        use super::trusted;

        Self::new(&Trusted::new(trusted::Config::default()), online)
    }

    /// Hash the secret to a public id.
    pub fn secret_to_id(secret: U256) -> U256 {
        digest::digest(&digest::SHA256, secret.as_byte_slice()).as_ref().into()
    }

    fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg> {
        let mut out = vec![];
        if self.online {
            debug!("Processing message {input:?}");
            match &input.msg {
                Msg::Ping => out.push(NodeMsg {
                    from: self.id,
                    to: input.from,
                    msg: Msg::Pong,
                }),
                Msg::Pong => info!("Got pong {input:?}"),
            }
        }
        out
    }
}

impl Module for Node {
    fn tick(&mut self, time: u64) -> Vec<BrokerAction> {
        if time > self.last + 1_000_000 {
            info!("One second later");
        }
        self.last = time;
        vec![]
    }

    fn action(&mut self, task: BrokerAction) -> Vec<BrokerAction> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::simul::trusted::{Config, Trusted};

    use super::*;

    #[test]
    fn test_receive() {
        let mut node1 = Node::dummy(true);
        let mut node2 = Node::dummy(true);
        node1.tick(100);
        assert_eq!(100u64, node1.last);
        let mut msgs = node1.receive(NodeMsg {
            from: node2.id,
            to: node1.id,
            msg: Msg::Ping,
        });
        assert_eq!(1, msgs.len());
        msgs = node2.receive(msgs.remove(0));
        assert_eq!(0, msgs.len());
    }
}
