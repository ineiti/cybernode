use serde::Serialize;
use utoipa::ToSchema;

use crate::simul::node_types::NodeID;

#[derive(ToSchema, Serialize)]
pub struct StatsReply {
   pub ids: Vec<NodeID>,
}
