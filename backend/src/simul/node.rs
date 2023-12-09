use primitive_types::U256;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use utoipa::ToSchema;

use super::{
    msgs::{Msg, NetworkMsg},
    network::RxTx,
};

/// Node is a simulation which can answer to certain messages.
/// If it receives regular 'tick's, it will send out some messages on its own.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: U256,
    pub mana: U256,
    last: u64,
}

impl Node {
    pub fn new(time: u64) -> Node {
        Node {
            id: rand::random::<[u8; 32]>().into(),
            mana: 0.into(),
            last: time,
        }
    }
}

impl RxTx for Node {
    fn receive(&mut self, input: Vec<NetworkMsg>) -> Vec<NetworkMsg> {
        let mut out = vec![];
        for nm in input {
            debug!("Processing message {nm:?}");
            match &nm.msg {
                Msg::Ping => out.push(NetworkMsg {
                    from: self.id,
                    to: nm.from,
                    msg: Msg::Pong,
                }),
                Msg::Pong => info!("Got pong {nm:?}"),
            }
        }
        out
    }

    fn tick(&mut self, time: u64) -> Vec<NetworkMsg> {
        if time > self.last + 1_000_000 {
            info!("One second later");
        }
        self.last = time;
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_receive() {
        let mut node1 = Node::new(0);
        let mut node2 = Node::new(0);
        node1.tick(100);
        assert_eq!(100u64, node1.last);
        let mut msgs = node1.receive(vec![NetworkMsg {
            from: node2.id,
            to: node1.id,
            msg: Msg::Ping,
        }]);
        assert_eq!(1, msgs.len());
        msgs = node2.receive(msgs);
        assert_eq!(0, msgs.len());
    }
}
