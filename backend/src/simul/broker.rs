// The broker interacts on one hand with the main module, and on
// the other hand it communicates with the network, simulation, and web
// module.

use std::{error::Error, sync::mpsc::Sender};

use primitive_types::U256;
use tracing::{info, warn};

use crate::simul::trusted::TrustedReply;

use super::{
    msgs::NodeAction,
    network::Network,
    node::{Node, NodeInfo, NodeMsg},
    simulator::{self, Simulator},
    trusted::{self, Trusted, TrustedRequest},
    web::Web,
};

pub struct Broker {
    simulator: Simulator,
    network: Network,
    web: Web,
    trusted: Sender<TrustedRequest>,
}

#[derive(Debug)]
pub enum BrokerMsg {
    Web(BMWeb),
    Network(BMNet),
    Simulator(BMSimul),
    Node(BMNode),
}

#[derive(Debug)]
pub enum BMNet {
    NodeOnline(U256, bool),
    NodeStatus(U256, bool),
    NodeAction(NodeAction),
    NodeMessage(NodeMsg),
    NodeAdd(Node),
}
#[derive(Debug)]
pub enum BMWeb {
    WebRegister(U256),
}

#[derive(Debug)]
pub enum BMSimul {}

#[derive(Debug)]
pub enum BMNode {}


impl From<BMNet> for BrokerMsg {
    fn from(value: BMNet) -> Self {
        BrokerMsg::Network(value)
    }
}

impl From<BMWeb> for BrokerMsg {
    fn from(value: BMWeb) -> Self {
        BrokerMsg::Web(value)
    }
}

impl From<BMSimul> for BrokerMsg {
    fn from(value: BMSimul) -> Self {
        BrokerMsg::Simulator(value)
    }
}

impl From<BMNode> for BrokerMsg {
    fn from(value: BMNode) -> Self {
        BrokerMsg::Node(value)
    }
}

pub struct NetworkStatus {}

impl Broker {
    pub fn new(trust: trusted::Config, sim: simulator::Config) -> Result<Self, Box<dyn Error>> {
        let trusted = Trusted::new(trust);
        let nodes: Vec<Node> = (0..sim.nodes_root + sim.nodes_flex)
            .map(|_| Node::new(&trusted, false))
            .collect();
        let node_ids = nodes.iter().map(|n| n.id()).collect();
        Ok(Self {
            simulator: Simulator::new(sim, node_ids)?,
            network: Network::new(),
            web: Web::new(trusted.clone()),
            trusted,
        })
    }

    pub fn default() -> Result<Self, Box<dyn Error>> {
        Self::new(trusted::Config::default(), simulator::Config::default())
    }

    pub fn tick(&mut self, time: u64) {
        let mut actions = self.simulator.tick(time);
        actions.append(&mut self.web.tick(time));
        actions.append(&mut self.network.tick(time));
        self.handle_msgs(actions);
    }

    /// Registers the given node identified by the secret.
    /// It returns the corresponding node-id.
    pub fn register(&mut self, secret: U256) -> U256 {
        info!("register");
        let msgs = self.web.action(BMWeb::WebRegister(secret));
        self.handle_msgs(msgs);
        Node::secret_to_id(secret)
    }

    /// Returns the NodeInfo for this given id.
    pub fn get_node_info(&mut self, id: U256) -> Result<NodeInfo, Box<dyn Error>> {
        let reply = trusted::TReqMsg::Info(id).send(&self.trusted)?;
        if let TrustedReply::NodeInfo(Some(ni)) = reply {
            return Ok(ni);
        }
        Err("No NodeInfo for this node available.".into())
    }

    fn handle_msgs(&mut self, mut msgs: Vec<BrokerMsg>) {
        while let Some(msg) = msgs.pop() {
            match msg {
                BrokerMsg::Web(msg) => msgs.append(&mut self.web.action(msg)),
                BrokerMsg::Network(msg) => msgs.append(&mut self.network.action(msg)),
                BrokerMsg::Simulator(msg) => msgs.append(&mut self.simulator.action(msg)),
                BrokerMsg::Node(_) => warn!("Got {msg:?} for node"),
            }
        }
    }
}
