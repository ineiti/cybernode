// The broker interacts on one hand with the main module, and on
// the other hand it communicates with the network, simulation, and web
// module.

use std::{error::Error, sync::mpsc::Sender};

use tracing::{error, info, warn};

use crate::simul::trusted::TrustedReply;

use super::{
    msgs::NodeAction,
    network::Network,
    node::{Node, NodeInfo},
    simulator::{self, Simulator},
    trusted::{self, TReqMsg, Trusted, TrustedRequest},
    web::Web, node_types::{NodeSecret, NodeID, Mana},
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
    NodeAction(NodeAction),
    NodeAdd(Node),
    NodeDel(NodeID),
}
#[derive(Debug)]
pub enum BMWeb {
    WebRegister(NodeSecret),
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
    pub fn new(
        trust: trusted::Config,
        sim: simulator::Config,
        now: u128,
    ) -> Result<Self, Box<dyn Error>> {
        let trusted = Trusted::new(trust, now);
        let nodes: Vec<Node> = (0..sim.nodes_root + sim.nodes_flex)
            .map(|_| Node::new(&trusted))
            .collect();
        let node_ids = nodes.iter().map(|n| n.id()).collect();
        Ok(Self {
            simulator: Simulator::new(sim, node_ids, trusted.clone())?,
            network: Network::new(),
            web: Web::new(trusted.clone()),
            trusted,
        })
    }

    pub fn default(now: u128) -> Result<Self, Box<dyn Error>> {
        Self::new(
            trusted::Config::default(),
            simulator::Config::default(),
            now,
        )
    }

    pub fn tick(&mut self, time: u128) {
        let mut actions = self.simulator.tick(time);
        actions.append(&mut self.web.tick(time));
        actions.append(&mut self.network.tick(time));
        self.handle_msgs(actions);
        if let Err(e) = TReqMsg::Tick(time).send(&self.trusted) {
            error!("While sending tick to Trusted: {e:?}");
        }
    }

    /// Registers the given node identified by the secret.
    /// It returns the corresponding node-id.
    pub fn register(&mut self, secret: NodeSecret) -> NodeID {
        info!("register");
        let msgs = self.web.action(BMWeb::WebRegister(secret));
        self.handle_msgs(msgs);
        secret.into()
    }

    /// Updates the mana of the node, and marks it as still connected.
    /// It returns how much mana the node currenlty has.
    /// TODO: perhaps it should return the NodeInfo?
    pub fn alive(&mut self, id: NodeID) -> Result<Mana, Box<dyn Error>> {
        info!("alive {id}");
        match TReqMsg::Alive(id).send(&self.trusted)? {
            TrustedReply::Mana(m) => return Ok(m),
            msg => return Err(format!("Got wrong type of message: {msg:?}").into()),
        }
    }

    /// Returns the NodeInfo for this given id.
    pub fn get_node_info(&mut self, id: NodeID) -> Result<NodeInfo, Box<dyn Error>> {
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
