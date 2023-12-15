use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use primitive_types::U256;
use tracing::{debug, error};

use super::node::{Node, NodeMsg};

pub struct Network {
    nodes: HashMap<U256, Node>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn tick(&mut self, now: u64) {
        let mut msgs = vec![];
        for node in self.nodes.iter_mut() {
            // msgs.append(&mut node.1.tick(now));
        }
        self.process_msgs(msgs);
    }

    pub fn node_add(&mut self, id: &U256, n: Node) {
        self.nodes.insert(*id, n);
    }

    pub fn node_del(&mut self, id: &U256) {
        self.nodes.remove(id);
    }

    fn process_msgs(&mut self, mut msgs: Vec<NodeMsg>) {
        while msgs.len() > 0 {
            debug!("Processing {} messages.", msgs.len());
            let msg = msgs.remove(0);
            msgs.append(&mut self.send_msg(msg));
        }
    }

    // Sends a message to the corresponding node.
    // If the node is in 'nodes_flex' and it's offline, the message will silently
    // be dropped.
    fn send_msg(&mut self, msg: NodeMsg) -> Vec<NodeMsg> {
        if let Some(node) = self.nodes.get_mut(&msg.to) {
            // return node.receive(msg);
        }
        vec![]
    }
}

#[cfg(test)]
mod test {}
