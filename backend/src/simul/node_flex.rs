use std::{
    error::Error,
    sync::mpsc::{Receiver, Sender},
};

use rand::random;

use super::{
    node::{Node, NodeMsg},
    node_basic::NodeBasic,
    trusted::Trusted, msgs::{NodeAction, NodeRequest},
};

pub struct NodeFlex {
    online: bool,
    p_out: u16,
    node: NodeBasic,
}

impl NodeFlex {
    pub fn random(trusted: &Sender<Trusted>, p_out: u16) -> Self {
        Self {
            online: false,
            node: NodeBasic::new(trusted, 0),
            p_out,
        }
    }

    pub fn go_online(&mut self) -> Result<Box<dyn Node>, Box<dyn Error>> {
        if self.online {
            return Err("Already online".into());
        }
        self.online = true;
        Ok(self.clone())
    }

    fn clone(&mut self) -> Box<dyn Node> {
        todo!()
    }

    pub fn is_online(&self) -> bool {
        self.online
    }
}

struct NodeFlexRxTx {
    tx: Sender<NFTx>,
    rx: Receiver<NFRx>,
    p_out: u16,
}

impl NodeFlexRxTx {}

impl Node for NodeFlexRxTx {
    fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg> {
        todo!()
    }

    fn tick(&mut self, time: u64) -> Vec<NodeMsg> {
        if random::<u16>() < self.p_out {}
        todo!()
    }

    fn action(&mut self, task: NodeAction) -> Vec<NodeRequest> {
        todo!()
    }
}

enum NFTx {}
enum NFRx {}
