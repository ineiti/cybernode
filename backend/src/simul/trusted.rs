use std::{
    collections::HashMap,
    error::Error,
    sync::mpsc::{self, channel, Receiver, Sender},
    thread,
};

use tracing::{debug, error, info, trace, warn};

use super::{node::NodeInfo, node_types::{NodeID, Mana}};

/// Trusted is a blockchain simulation.
/// In the simulation it replaces a central server with global knowledge.
/// To avoid too many synchronisation problems, Trusted exposes a receiver
/// channel where requests go in.
/// Every request also contains a receiver channel for the answer.
///
/// Currently, Trusted is responsible for the following:
/// - mark nodes as inactive if no 'active' message is received
/// - increase mana for active nodes (1 / s)
/// - decrease mana for inactive nodes (1 / (86_400 * 7 / 3_600)s)
///   This means a node running for 1h stays in the list for 1 week
/// - clean up nodes once they reach 0 mana

pub struct Trusted {
    // The configuration of this Trusted service
    config: Config,
    // List of known nodes
    nodes: HashMap<NodeID, NodeData>,
    // Nodes can send requests here
    ch_request_rx: Receiver<TrustedRequest>,
    // Last mana increase
    last_mana_inc: u128,
    // Last mana decrease
    last_mana_dec: u128,
    // Latest tick time
    last_tick_time: u128,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub time_mana_increase: u128,
    pub time_mana_decrease: u128,
    pub time_node_active: u128,
}

const TIME_SECOND: u128 = 1_000;

impl Config {
    pub fn default() -> Self {
        Self {
            time_mana_increase: TIME_SECOND,
            time_mana_decrease: (86_400 * 7 * TIME_SECOND / 3_600),
            time_node_active: 60 * TIME_SECOND,
        }
    }
}

impl Trusted {
    /// Create a new trusted service.
    /// Communication happens through the returned channel.
    pub fn new(config: Config, now: u128) -> Sender<TrustedRequest> {
        let (ch_request_tx, ch_request_rx) = mpsc::channel::<TrustedRequest>();
        thread::spawn(move || {
            Self {
                config,
                nodes: HashMap::new(),
                ch_request_rx,
                last_mana_inc: now,
                last_mana_dec: now,
                last_tick_time: now,
            }
            .listen();
        });
        ch_request_tx
    }

    /// Creates a new trusted service with default values.
    pub fn new_default(now: u128) -> Sender<TrustedRequest> {
        Self::new(Config::default(), now)
    }

    fn listen(&mut self) {
        loop {
            match self.ch_request_rx.recv() {
                Ok(msg) => match &msg.message {
                    TReqMsg::Register(ni) => {
                        debug!("Registering node {ni}");
                        self.nodes.insert(
                            ni.id,
                            NodeData {
                                info: ni.clone(),
                                active_until: self.last_tick_time + self.config.time_node_active,
                            },
                        );
                        msg.reply(TrustedReply::NodeList(self.get_node_list()));
                    }
                    TReqMsg::Close => {
                        warn!("Closing Trusted");
                        return;
                    }
                    TReqMsg::Tick(now) => {
                        self.tick(*now);
                        msg.reply(TrustedReply::OK);
                    }
                    TReqMsg::Alive(id) => msg.reply(self.alive(id)),
                    TReqMsg::Info(id) => {
                        trace!("Got asked for node {id}");
                        msg.reply(TrustedReply::NodeInfo(
                            self.nodes.get(id).map(|n| n.info.clone()),
                        ))
                    }
                },
                Err(e) => {
                    info!("Trusted listener closed with error: {e}");
                    return;
                }
            }
        }
    }

    fn get_node_list(&self) -> Vec<NodeInfo> {
        self.nodes.values().map(|nd| nd.info.clone()).collect()
    }

    fn tick(&mut self, now: u128) {
        // Increase the mana for all the nodes which have been active more recent than
        // config.time_node_active.
        let mana_inc = (now - self.last_mana_inc) / self.config.time_mana_increase;
        if mana_inc > 0 {
            self.nodes.iter_mut().for_each(|(_, n)| {
                if n.is_active(now) {
                    n.info.mana += mana_inc.into();
                }
            });
            self.last_mana_inc += mana_inc * self.config.time_mana_increase;
        }

        // Decrease the mana for all the nodes which have been inactive.
        let mana_dec = (now - self.last_mana_dec) / self.config.time_mana_decrease;
        if mana_dec > 0 {
            let mut nodes_rem = vec![];
            self.nodes.iter_mut().for_each(|(id, n)| {
                if !n.is_active(now) {
                    if n.info.mana >= mana_dec.into() {
                        n.info.mana -= mana_dec.into();
                    } else {
                        n.info.mana = 0.into();
                        nodes_rem.push(id.clone());
                    }
                }
            });
            self.last_mana_dec += mana_dec * self.config.time_mana_decrease;
            for node in nodes_rem{
                self.nodes.remove(&node);
            }
        }

        self.last_tick_time = now;
    }

    /// Every node should call this from time to time in order to be kept alive.
    /// Else the node will be marked as 'inactive', and it will start losing its
    /// mana.
    fn alive(&mut self, id: &NodeID) -> TrustedReply {
        if let Some(node) = self.nodes.get_mut(id) {
            node.active_until = self.last_tick_time + self.config.time_node_active;
            TrustedReply::Mana(node.info.mana)
        } else {
            TrustedReply::ErrorMsg("Node not registered".into())
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
    fn reply(&self, reply: TrustedReply) {
        if let Err(e) = self.reply.send(reply) {
            error!("While sending reply: {e}");
        }
    }
}

#[derive(Debug, Clone)]
pub enum TReqMsg {
    /// Registers a new node
    Register(NodeInfo),
    /// Mark node as alive for the next x ticks
    Alive(NodeID),
    /// Update mana - increase for online nodes, decrease for offline nodes
    Tick(u128),
    /// Get NodeInfo of a node
    Info(NodeID),
    /// Close the channel and stop
    Close,
}

impl TReqMsg {
    pub fn send(&self, trusted: &Sender<TrustedRequest>) -> Result<TrustedReply, Box<dyn Error>> {
        let (tx, rx) = channel();
        trusted.send(TrustedRequest {
            message: self.clone(),
            reply: tx,
        })?;

        Ok(rx.recv()?)
    }
}

#[derive(Debug)]
pub enum TrustedReply {
    NodeList(Vec<NodeInfo>),
    NodeInfo(Option<NodeInfo>),
    Mana(Mana),
    OK,
    ErrorMsg(String),
}

#[derive(Debug)]
struct NodeData {
    info: NodeInfo,
    active_until: u128,
}

impl NodeData {
    fn is_active(&self, now: u128) -> bool {
        self.active_until >= now
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type ResErr = Result<(), Box<dyn Error>>;

    #[test]
    fn test_register() -> ResErr {
        let tr = Trusted::new_default(0);
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
        let mut now = 0u128;
        let cfg = Config::default();
        let tr = Trusted::new(cfg.clone(), now);

        // Returns an error on "Alive" if the node doesn't exist
        let node = NodeInfo::random();
        let reply = TReqMsg::Alive(node.id).send(&tr)?;
        assert_matches!(reply, TrustedReply::ErrorMsg(_));

        // Registering and asking for the node works
        TReqMsg::Register(node.clone()).send(&tr)?;
        let reply = TReqMsg::Alive(node.id).send(&tr)?;
        assert_matches!(reply, TrustedReply::Mana(m) if m == 0.into());

        // Mana is increased if the time passes
        now += cfg.time_mana_increase;
        TReqMsg::Tick(now).send(&tr)?;
        let reply = TReqMsg::Alive(node.id).send(&tr)?;
        let mut mana = 1;
        assert_matches!(reply, TrustedReply::Mana(m) if m == mana.into());

        // Mana increases as long as the node is active
        now += cfg.time_mana_increase;
        TReqMsg::Tick(now).send(&tr)?;
        let reply = TReqMsg::Alive(node.id).send(&tr)?;
        mana += 1;
        assert_matches!(reply, TrustedReply::Mana(m) if m == mana.into());
        now += cfg.time_node_active;
        TReqMsg::Tick(now).send(&tr)?;
        let reply = TReqMsg::Info(node.id).send(&tr)?;
        mana += cfg.time_node_active / cfg.time_mana_increase;
        assert_matches!(reply, TrustedReply::NodeInfo(Some(ni)) if ni.mana == mana.into());

        // Mana decreases when the node is inactive
        now += cfg.time_mana_decrease;
        TReqMsg::Tick(now).send(&tr)?;
        let reply = TReqMsg::Info(node.id).send(&tr)?;
        mana -= 1;
        assert_matches!(reply, TrustedReply::NodeInfo(Some(ni)) if ni.mana == mana.into());

        // Node is removed when mana is used up
        now += cfg.time_mana_decrease * mana;
        TReqMsg::Tick(now).send(&tr)?;
        let reply = TReqMsg::Info(node.id).send(&tr)?;
        assert_matches!(reply, TrustedReply::NodeInfo(Some(_)));
        now += cfg.time_mana_decrease;
        TReqMsg::Tick(now).send(&tr)?;
        let reply = TReqMsg::Info(node.id).send(&tr)?;
        assert_matches!(reply, TrustedReply::NodeInfo(None));

        Ok(())
    }
}
