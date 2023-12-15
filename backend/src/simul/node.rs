use std::sync::mpsc::Sender;

use primitive_types::U256;
use tracing::{debug, info};

use super::{trusted::Trusted, msgs::{NodeAction, NodeRequest}};

pub trait Node: Send {
    fn action(&mut self, task: NodeAction) -> Vec<NodeRequest>;
    fn receive(&mut self, input: NodeMsg) -> Vec<NodeMsg>;
    fn tick(&mut self, time: u64) -> Vec<NodeMsg>;
}

#[derive(Debug)]
pub struct NodeMsg {
    pub from: U256,
    pub to: U256,
    pub msg: Msg,
}

#[derive(Debug)]
pub enum Msg {
    Ping,
    Pong,
}
