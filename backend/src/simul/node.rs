use byte_slice_cast::*;
use std::sync::mpsc::Sender;

use primitive_types::U256;
use ring::digest;
use tracing::{debug, error, info};

use super::{
    broker::{BrokerAction, Module},
    trusted::{NodeInfo, TReqMsg, TrustedRequest},
};

/// Node is a simulation which can answer to certain messages.
/// If it receives regular 'tick's, it will send out some messages on its own.
#[derive(Debug)]
pub struct Node {
    info: NodeInfo,
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
        Self::from_info(NodeInfo::random(), trusted, online)
    }

    /// Make a dummy node with a dummy channel to Trusted.
    #[cfg(test)]
    pub fn dummy(online: bool) -> Self {
        use super::trusted;
        use crate::simul::trusted::Trusted;

        Self::new(&Trusted::new(trusted::Config::default()), online)
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

    fn update_trusted(&self) {
        if let Err(e) = TReqMsg::Register(self.info.clone()).send(&self.trusted) {
            error!("While registering node: {e:?}");
        }
    }

    fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg> {
        let mut out = vec![];
        if self.online {
            debug!("Processing message {input:?}");
            match &input.msg {
                Msg::Ping => out.push(NodeMsg {
                    from: self.id(),
                    to: input.from,
                    msg: Msg::Pong,
                }),
                Msg::Pong => info!("Got pong {input:?}"),
            }
        }
        out
    }

    pub fn from_info(info: NodeInfo, trusted: &Sender<TrustedRequest>, online: bool) -> Self {
        let reply = Self {
            info,
            online,
            trusted: trusted.clone(),
        };
        reply.update_trusted();
        reply
    }
}

impl Module for Node {
    fn tick(&mut self, time: u64) -> Vec<BrokerAction> {
        if time > self.info.last_seen + 1_000_000 {
            info!("One second later");
        }
        self.info.last_seen = time;
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
