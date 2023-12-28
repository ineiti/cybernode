use std::collections::HashMap;

use primitive_types::U256;
use tracing::{debug, trace};

use super::{
    broker::{BMNet, BrokerMsg},
    node::{Node, NodeMsg},
};

pub struct Network {
    nodes: HashMap<U256, Node>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn action(&mut self, action: BMNet) -> Vec<BrokerMsg> {
        match action {
            BMNet::NodeAdd(n) => {
                if !self.nodes.contains_key(&n.id()) {
                    debug!("Adding node {}", n.info());
                    self.nodes.insert(n.id(), n);
                } else {
                    debug!("Node already present: {}", n.info());
                }
            }
            _ => {}
        }
        vec![]
    }

    pub fn tick(&mut self, now: u128) -> Vec<BrokerMsg> {
        let mut msgs = vec![];
        for node in self.nodes.values_mut() {
            msgs.append(&mut node.tick(now));
        }
        self.process_msgs(msgs);

        vec![]
    }

    fn process_msgs(&mut self, mut msgs: Vec<NodeMsg>) {
        debug!("Processing {} messages.", msgs.len());
        while let Some(msg) = msgs.pop() {
            msgs.append(&mut self.send_msg(msg));
        }
    }

    // Sends a message to the corresponding node.
    // If the node is in 'nodes_flex' and it's offline, the message will silently
    // be dropped.
    fn send_msg(&mut self, msg: NodeMsg) -> Vec<NodeMsg> {
        if let Some(node) = self.nodes.get_mut(&msg.to) {
            trace!("Sending {msg:?}");
            return node.receive(msg);
        }
        vec![]
    }
}

#[cfg(test)]
mod test {}
