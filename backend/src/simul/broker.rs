// The broker interacts on one hand with the main module, and on
// the other hand it communicates with the network, simulation, and web
// module.

use primitive_types::U256;

use super::node::Node;

pub struct Broker {}

pub trait Module {
    fn action(&mut self, action: BrokerAction) -> Vec<BrokerAction>;
    fn tick(&mut self, time: u64) -> Vec<BrokerAction>;
}

pub enum BrokerMsg {
    Action(BrokerAction),
    Status(NetworkStatus),
}

#[derive(Debug)]
pub enum BrokerAction {
    NodeOnline(U256, bool),
    NodeStatus(U256, bool),
    NodeAction,
    NodeAdd(Node),
}

pub struct NetworkStatus {}

impl Broker {}