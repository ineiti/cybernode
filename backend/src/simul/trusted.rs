use std::{
    collections::HashMap,
    error::Error,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use primitive_types::U256;
use serde::Serialize;
use tracing::{debug, error, info};

/// Trusted is a blockchain simulation.
/// In the simulation it replaces a central server with global knowledge.
/// To avoid too many synchronisation problems, Trusted exposes a receiver
/// channel where requests go in.
/// Every request also contains a receiver channel for the answer.

pub struct Trusted {
    // The configuration of this Trusted service
    config: Config,
    // List of nodes with a timestamp of the latest appearance.
    nodes: HashMap<U256, NodeInfo>,
    // Nodes can send requests here
    ch_request_rx: Receiver<TrustedRequest>,
    // Number of ticks received so far
    ticks: u64,
    // When the last tick was called
    last_tick_time: u64,
    // Last mana increase
    last_tick_mana: u64,
}

pub struct Config {
    pub tick_interval: u64,
    pub tick_mana_increase: u64,
    pub time_node_cleanup: u64,
}

impl Config {
    pub fn default() -> Self {
        Self {
            tick_interval: 1_000_000,
            tick_mana_increase: 1,
            time_node_cleanup: 600,
        }
    }
}

impl Trusted {
    /// Create a new trusted service.
    /// Communication happens through the returned channel.
    pub fn new(config: Config) -> Sender<TrustedRequest> {
        let (ch_request_tx, ch_request_rx) = mpsc::channel::<TrustedRequest>();
        thread::spawn(|| {
            Self {
                config,
                nodes: HashMap::new(),
                ch_request_rx,
                ticks: 0,
                last_tick_time: 0,
                last_tick_mana: 0,
            }
            .listen();
        });
        ch_request_tx
    }

    /// Creates a new trusted service with default values.
    pub fn new_default() -> Sender<TrustedRequest> {
        Self::new(Config::default())
    }

    fn listen(&mut self) {
        loop {
            match self.ch_request_rx.recv() {
                Ok(msg) => match &msg.message {
                    TReqMsg::Register(ni) => {
                        self.nodes.insert(ni.id, ni.clone());
                        msg.send(TrustedReply::NodeList(self.get_node_list()));
                    }
                    TReqMsg::Close => {
                        debug!("Closing Trusted");
                        return;
                    }
                    TReqMsg::Tick(now) => {
                        self.tick(now);
                        msg.send(TrustedReply::OK);
                    }
                    TReqMsg::Alive(id) => msg.send(self.alive(id)),
                    TReqMsg::Info(id) => msg.send(TrustedReply::NodeInfo(
                        self.nodes.get(id).map(|n| n.clone()),
                    )),
                },
                Err(e) => {
                    info!("Trusted listener closed with error: {e}");
                    return;
                }
            }
        }
    }

    fn get_node_list(&self) -> Vec<NodeInfo> {
        self.nodes.values().cloned().collect()
    }

    fn tick(&mut self, now: &u64) {
        self.ticks += 1;
        if self.ticks == self.last_tick_mana + self.config.tick_mana_increase {
            self.nodes.iter_mut().for_each(|(_, n)| n.mana += 1.into());
            self.last_tick_mana = self.ticks;
        }
        if self.ticks >= self.config.time_node_cleanup {
            let too_old = now - self.config.time_node_cleanup;
            self.nodes.retain(|_, v| v.last_seen >= too_old);
        }
        self.last_tick_time = *now;
    }

    fn alive(&mut self, id: &U256) -> TrustedReply {
        match self.nodes.get_mut(id) {
            Some(node) => {
                node.last_seen = self.last_tick_time;
                TrustedReply::Mana(node.mana)
            }
            None => TrustedReply::ErrorMsg("Node not registered".into()),
        }
    }

    /// Static method for simplified querying of the Trusted service.
    /// This creates the necessary channel, sends the request, and returns the result.
    pub fn send(ch: &Sender<TrustedRequest>, req: TReqMsg) -> Result<TrustedReply, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel::<TrustedReply>();
        ch.send(TrustedRequest {
            message: req,
            reply: tx,
        })?;
        Ok(rx.recv()?)
    }
}

#[derive(Debug)]
pub struct TrustedRequest {
    pub message: TReqMsg,
    pub reply: Sender<TrustedReply>,
}

impl TrustedRequest {
    fn send(&self, reply: TrustedReply) {
        if let Err(e) = self.reply.send(reply) {
            error!("While sending reply: {e}");
        }
    }
}

#[derive(Debug)]
pub enum TReqMsg {
    Register(NodeInfo),
    Alive(U256),
    Tick(u64),
    Info(U256),
    Close,
}

#[derive(Debug)]
pub enum TrustedReply {
    NodeList(Vec<NodeInfo>),
    NodeInfo(Option<NodeInfo>),
    Mana(U256),
    OK,
    ErrorMsg(String),
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfo {
    pub id: U256,
    pub last_seen: u64,
    pub mana: U256,
}

impl NodeInfo {
    pub fn random() -> Self {
        Self {
            id: rand::random::<[u8; 32]>().into(),
            last_seen: 0,
            mana: U256::zero(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type ResErr = Result<(), Box<dyn Error>>;

    #[test]
    fn test_register() -> ResErr {
        let tr = Trusted::new_default();
        let node1 = NodeInfo::random();
        let reply = Trusted::send(&tr, TReqMsg::Register(node1.clone()))?;
        assert_matches!(reply, TrustedReply::NodeList(reply) if reply.len() == 1);
        let reply = Trusted::send(&tr, TReqMsg::Register(node1))?;
        assert_matches!(reply, TrustedReply::NodeList(reply) if reply.len() == 1);

        let node2 = NodeInfo::random();
        let reply = Trusted::send(&tr, TReqMsg::Register(node2))?;
        assert_matches!(reply, TrustedReply::NodeList(reply) if reply.len() == 2);
        Ok(())
    }

    #[test]
    fn test_alive() -> ResErr {
        let tr = Trusted::new_default();
        let node = NodeInfo::random();
        let reply = Trusted::send(&tr, TReqMsg::Alive(node.id))?;
        assert_matches!(reply, TrustedReply::ErrorMsg(_));

        Trusted::send(&tr, TReqMsg::Register(node.clone()))?;
        let reply = Trusted::send(&tr, TReqMsg::Alive(node.id))?;
        assert_matches!(reply, TrustedReply::Mana(m) if m == 0.into());

        Trusted::send(&tr, TReqMsg::Tick(Config::default().tick_interval))?;
        let reply = Trusted::send(&tr, TReqMsg::Alive(node.id))?;
        assert_matches!(reply, TrustedReply::Mana(m) if m == 1.into());
        Ok(())
    }
}
