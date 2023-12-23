use std::collections::HashMap;

use primitive_types::U256;
use tracing::{debug, trace};

use super::{
    broker::{BrokerAction, Module},
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

    fn process_msgs(&mut self, mut msgs: Vec<BrokerAction>) {
        while msgs.len() > 0 {
            debug!("Processing {} messages.", msgs.len());
            if let BrokerAction::NodeMessage(msg) = msgs.remove(0) {
                for m in self.send_msg(msg) {
                    msgs.push(BrokerAction::NodeMessage(m));
                }
            }
        }
    }

    // Sends a message to the corresponding node.
    // If the node is in 'nodes_flex' and it's offline, the message will silently
    // be dropped.
    fn send_msg(&mut self, msg: NodeMsg) -> Vec<NodeMsg> {
        if let Some(node) = self.nodes.get_mut(&msg.to) {
            trace!("Got node {}", node.info());
            // return node.receive(msg);
        }
        vec![]
    }
}

impl Module for Network {
    fn action(&mut self, action: BrokerAction) -> Vec<BrokerAction> {
        match action {
            BrokerAction::NodeAdd(n) => {
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

    fn tick(&mut self, now: u64) -> Vec<BrokerAction> {
        let mut msgs = vec![];
        for node in self.nodes.values_mut() {
            msgs.append(&mut node.tick(now));
        }
        self.process_msgs(msgs);

        vec![]
    }
}

#[cfg(test)]
mod test {}
