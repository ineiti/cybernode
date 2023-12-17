// The broker interacts on one hand with the main module, and on
// the other hand it communicates with the network, simulation, and web
// module.

use std::{
    error::Error,
    sync::mpsc::{channel, Sender},
};

use primitive_types::U256;
use tracing::warn;

use crate::simul::trusted::TrustedReply;

use super::{
    msgs::NodeAction,
    network::Network,
    node::{Node, NodeMsg},
    simulator::{self, Simulator},
    trusted::{self, NodeInfo, Trusted, TrustedRequest},
    web::Web,
};

pub struct Broker {
    simulator: Box<dyn Module>,
    network: Box<dyn Module>,
    web: Box<dyn Module>,
    trusted: Sender<TrustedRequest>,
}

pub trait Module {
    fn action(&mut self, action: BrokerAction) -> Vec<BrokerAction>;
    fn tick(&mut self, time: u64) -> Vec<BrokerAction>;
}

pub enum BrokerMsg {
    Action(BrokerAction),
    Status(NetworkStatus),
    Tick(u64),
}

#[derive(Debug)]
pub enum BrokerAction {
    NodeOnline(U256, bool),
    NodeStatus(U256, bool),
    NodeAction(NodeAction),
    NodeMessage(NodeMsg),
    NodeAdd(Node),
    WebRegister(U256),
}

pub struct NetworkStatus {}

impl Broker {
    pub fn new(trust: trusted::Config, sim: simulator::Config) -> Result<Self, Box<dyn Error>> {
        let trusted = Trusted::new(trust);
        let nodes: Vec<Node> = (0..sim.nodes_root + sim.nodes_flex)
            .map(|_| Node::new(&trusted, false))
            .collect();
        let node_ids = nodes.iter().map(|n| n.id).collect();
        Ok(Self {
            simulator: Box::new(Simulator::new(sim, node_ids)?),
            network: Box::new(Network::new()),
            web: Box::new(Web::new()),
            trusted,
        })
    }

    pub fn default() -> Result<Self, Box<dyn Error>> {
        Self::new(trusted::Config::default(), simulator::Config::default())
    }

    pub fn tick(&mut self, time: u64) -> Vec<BrokerAction> {
        let mut actions = self.simulator.tick(time);
        actions.append(&mut self.web.tick(time));
        let mut answer = self.network.tick(time);
        for action in actions {
            answer.append(&mut self.network.action(action));
        }
        answer
    }

    /// Registers the given node identified by the secret.
    /// It returns the corresponding node-id.
    pub fn register(&mut self, secret: U256) -> U256 {
        self.action_web(BrokerAction::WebRegister(secret));
        Node::secret_to_id(secret)
    }

    /// Returns the NodeInfo for this given id.
    pub fn get_node_info(&mut self, id: U256) -> Result<NodeInfo, Box<dyn Error>> {
        let (tx, rx) = channel();
        self.trusted.send(TrustedRequest {
            message: trusted::TReqMsg::Info(id),
            reply: tx,
        })?;
        if let TrustedReply::NodeInfo(Some(ni)) = rx.recv()? {
            return Ok(ni);
        }
        Err("No NodeInfo for this node available.".into())
    }

    fn action_web(&mut self, action: BrokerAction) {
        for a in self.web.action(action) {
            self.action_net(a);
        }
    }

    fn action_net(&mut self, action: BrokerAction) {
        let actions = self.network.action(action);
        if actions.len() > 0 {
            warn!("Got some actions returned from the network module: {actions:?}");
        }
    }
}
