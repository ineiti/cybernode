// The broker interacts on one hand with the main module, and on
// the other hand it communicates with the network, simulation, and web
// module.

use primitive_types::U256;

use super::node::Node;

pub struct Broker {}

pub enum BrokerMsg {
    Action(BrokerAction),
    Status(NetworkStatus),
}

pub enum BrokerAction {
    NodeStatus(U256, bool),
    NodeAction,
    NodeAdd(Node),
}

pub struct NetworkStatus {}

impl Broker {}