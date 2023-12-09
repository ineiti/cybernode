use serde::Serialize;
use utoipa::ToSchema;
use primitive_types::U256;

#[derive(ToSchema, Serialize)]
pub struct StatsReply {
   pub ids: Vec<U256>,
}
