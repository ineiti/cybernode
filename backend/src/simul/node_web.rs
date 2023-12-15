use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use primitive_types::U256;
use tracing::{error, info};

use super::{node::{Node, NodeMsg}, node_basic::NodeBasic, trusted::Trusted, msgs::{NodeAction, NodeRequest}};

pub struct NodeWeb {
    tx: Sender<NWRequest>,
    id: U256,
}

pub struct NWRequest {
    msg: NWMsg,
    tx: Sender<NWReply>,
}

pub enum NWMsg {
    Tick(u64),
    Messages(Vec<NodeMsg>),
}

pub enum NWReply {
    Messages(Vec<NodeMsg>),
}

impl NodeWeb {
    pub fn new(trusted: Sender<Trusted>) -> Self {
        let (tx, rx) = mpsc::channel();
        let node = NodeBasic::new(&trusted, 0);
        let id = node.id;
        thread::spawn(|| Self::listen(rx, node));
        Self { tx, id }
    }

    pub fn get_tx(&self) -> Sender<NWRequest> {
        self.tx.clone()
    }

    fn listen(rx: Receiver<NWRequest>, n: NodeBasic) {
        loop {
            match rx.recv() {
                Ok(_) => todo!(),
                Err(e) => {
                    info!("Trusted listener closed with error: {e}");
                    return;
                }
            }
        }
    }

    fn send_msg(&self, msg: NWMsg) -> Vec<NodeMsg> {
        let (tx, rx) = mpsc::channel();
        match self.tx.send(NWRequest { msg, tx }) {
            Ok(_) => match rx.recv() {
                Ok(msgs) => match msgs {
                    NWReply::Messages(ms) => ms,
                },
                Err(e) => {
                    error!("Receiving error: {e}");
                    vec![]
                }
            },
            Err(e) => {
                error!("Couldn't send message: {e}");
                vec![]
            }
        }
    }
}

impl Node for NodeWeb {
    fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg> {
        todo!()
    }

    fn tick(&mut self, time: u64) -> Vec<NodeMsg> {
        todo!()
    }

    fn action(&mut self, task: NodeAction) -> Vec<NodeRequest> {
        todo!()
    }
}
