use std::sync::mpsc::Sender;

use primitive_types::U256;
use tracing::{debug, info};

use super::{trusted::Trusted, msgs::{NodeRequest, NodeAction}};

/// Node is a simulation which can answer to certain messages.
/// If it receives regular 'tick's, it will send out some messages on its own.
#[derive(Debug)]
pub struct Node {
    pub id: U256,
    pub mana: U256,
    last: u64,
    online: bool,
    trusted: Sender<Trusted>,
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
    pub fn new(trusted: &Sender<Trusted>, online: bool) -> Self {
        Self {
            id: rand::random::<[u8; 32]>().into(),
            mana: 0.into(),
            last: 0,
            online,
            trusted: trusted.clone(),
        }
    }

    #[cfg(test)]
    pub fn dummy(online: bool) -> Self {
        use std::sync::mpsc::channel;

        let (tx, _) = channel();
        Self::new(&tx, online)
    }

    fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg> {
        let mut out = vec![];
        debug!("Processing message {input:?}");
        match &input.msg {
            Msg::Ping => out.push(NodeMsg {
                from: self.id,
                to: input.from,
                msg: Msg::Pong,
            }),
            Msg::Pong => info!("Got pong {input:?}"),
        }
        out
    }

    fn tick(&mut self, time: u64) -> Vec<NodeMsg> {
        if time > self.last + 1_000_000 {
            info!("One second later");
        }
        self.last = time;
        vec![]
    }

    fn action(&mut self, task: NodeAction) -> Vec<NodeRequest> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_receive() {
        // let mut node1 = Node::new(0);
        // let mut node2 = Node::new(0);
        // node1.tick(100);
        // assert_eq!(100u64, node1.last);
        // let mut msgs = node1.receive(NodeMsg {
        //     from: node2.id,
        //     to: node1.id,
        //     msg: Msg::Ping,
        // });
        // assert_eq!(1, msgs.len());
        // msgs = node2.receive(msgs.remove(0));
        // assert_eq!(0, msgs.len());
    }
}
