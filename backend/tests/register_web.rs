use std::error::Error;
use tracing::info;
use test_log::test;

use backend::simul::broker::Broker;

#[test]
fn test_register() -> Result<(), Box<dyn Error>>{
    let mut broker = Broker::default().expect("Couldn't start broker");
    let secret = rand::random::<[u8; 32]>().into();
    let id = broker.register(secret);
    info!("Registered and got id: {id:#34x}");
    let info = broker.get_node_info(id)?;
    info!("Node info is: {info}");
    let id2 = broker.register(secret);
    assert_eq!(id, id2);

    Ok(())
}
