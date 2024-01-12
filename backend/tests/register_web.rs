use std::error::Error;
use tracing::info;
use test_log::test;

use backend::simul::{broker::Broker, node_types::NodeSecret};

#[test]
fn test_register() -> Result<(), Box<dyn Error>>{
    let mut broker = Broker::default(0).expect("Couldn't start broker");
    let secret = NodeSecret::random();
    let id = broker.register(secret);
    info!("Registered and got id: {id}");
    let info = broker.get_node_info(id)?;
    info!("Node info is: {info}");
    let id2 = broker.register(secret);
    assert_eq!(id, id2);

    Ok(())
}
