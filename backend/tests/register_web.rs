use std::error::Error;

use backend::simul::broker::Broker;

#[test]
fn test_register() -> Result<(), Box<dyn Error>>{
    let mut broker = Broker::default().expect("Couldn't start broker");
    let secret = rand::random::<[u8; 32]>().into();
    let id = broker.register(secret);
    println!("Registering and got id: {id:?}");
    let info = broker.get_node_info(id)?;
    println!("Node info is: {info:?}");

    Ok(())
}
