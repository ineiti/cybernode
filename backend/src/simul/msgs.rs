use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkMsg {
    pub from: U256,
    pub to: U256,
    pub msg: Msg,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Msg {
    Ping,
    Pong,
}
