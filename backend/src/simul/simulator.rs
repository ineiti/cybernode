use std::{error::Error, sync::mpsc::Sender};

use primitive_types::U256;
use rand::random;
use tracing::{error, trace};

use super::{
    broker::{BMNet, BMSimul, BrokerMsg},
    node::Node,
    trusted::{TReqMsg, TrustedReply, TrustedRequest},
};

pub struct Simulator {
    nodes: Vec<NodeFlex>,
    trusted: Sender<TrustedRequest>,
}

pub struct NodeFlex {
    id: U256,
    online: bool,
    p_sign_in: u16,
    p_sign_out: u16,
}

#[derive(Debug, Clone)]
pub struct Config {
    // Number of nodes always on.
    pub nodes_root: usize,
    // How many nodes in total come and go.
    pub nodes_flex: usize,
    // The probability (0..2**16-1) for an offline flex-node to go online after a tick.
    pub p_sign_in: u16,
    // The probability (0..2**16-1) for an online flex-node to go offline after a tick.
    pub p_sign_out: u16,
}

impl Config {
    pub fn default() -> Self {
        Self {
            nodes_root: 5,
            nodes_flex: 10,
            p_sign_in: 0x1000,
            p_sign_out: 0xa00,
        }
    }
}

impl Simulator {
    /// TODO: probably this'll need a Trusted tx-channel later on.
    pub fn new(
        config: Config,
        nodes: Vec<U256>,
        trusted: Sender<TrustedRequest>,
    ) -> Result<Self, Box<dyn Error>> {
        if nodes.len() != config.nodes_root + config.nodes_flex {
            return Err("wrong number of nodes".into());
        }
        Ok(Self {
            nodes: Self::node_flex(config, nodes),
            trusted,
        })
    }

    fn node_flex(config: Config, ids: Vec<U256>) -> Vec<NodeFlex> {
        let mut nf = Self::node_flex_part(ids[0..config.nodes_root].to_vec(), u16::MAX, 0);
        nf.append(&mut Self::node_flex_part(
            ids[config.nodes_root..].to_vec(),
            config.p_sign_in,
            config.p_sign_out,
        ));
        nf
    }

    fn node_flex_part(nodes: Vec<U256>, p_sign_in: u16, p_sign_out: u16) -> Vec<NodeFlex> {
        nodes
            .iter()
            .map(|n| NodeFlex {
                id: *n,
                online: false,
                p_sign_in,
                p_sign_out,
            })
            .collect()
    }

    pub fn action(&mut self, action: BMSimul) -> Vec<BrokerMsg> {
        match action {}
    }

    pub fn tick(&mut self, _time: u128) -> Vec<BrokerMsg> {
        let mut answer = vec![];

        for node in &mut self.nodes {
            if node.online && node.p_sign_out > 0 && node.p_sign_out > random::<u16>() {
                node.online = false;
                answer.push(BMNet::NodeDel(node.id).into());
            } else if node.online == false && node.p_sign_in > random::<u16>() {
                node.online = true;
                match TReqMsg::Info(node.id).send(&self.trusted) {
                    Ok(reply) => {
                        if let TrustedReply::NodeInfo(Some(ni)) = reply {
                            answer.push(BMNet::NodeAdd(Node::from_info(ni, &self.trusted)).into());
                        }
                    }
                    Err(_) => error!("Didn't find node {:?}", node.id),
                }
            }
            if node.online {
                if let Ok(TrustedReply::Mana(m)) = TReqMsg::Alive(node.id).send(&self.trusted){
                    trace!("Mana of node {}: {}", node.id, m);
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod test {
    use std::cmp::{max, min};
    use test_log::test;

    use super::*;
    use crate::simul::trusted::Trusted;

    impl Simulator {
        fn nodes_online(&self) -> usize {
            self.nodes.iter().filter(|n| n.online).count()
        }
    }

    #[test]
    fn test_online() -> Result<(), Box<dyn Error>> {
        let cfg = Config::default();
        let ids = (0..cfg.nodes_root + cfg.nodes_flex)
            .map(|_| rand::random::<[u8; 32]>().into())
            .collect();
        let trusted = Trusted::new_default(0);
        let mut simul = Simulator::new(cfg.clone(), ids, trusted)?;
        assert_eq!(0, simul.nodes_online());

        // Make sure that the number of nodes fluctuates somehow.
        while simul.nodes_online() < 10 {
            simul.tick(0);
        }
        let (mut min_nodes, mut max_nodes) = (cfg.nodes_root + cfg.nodes_flex, 0);
        for i in 1..100 {
            simul.tick(i);
            min_nodes = min(min_nodes, simul.nodes_online());
            max_nodes = max(max_nodes, simul.nodes_online());
        }
        assert!(min_nodes < 10);
        assert!(max_nodes > 10);

        Ok(())
    }
}
