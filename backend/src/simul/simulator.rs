use std::{error::Error, sync::mpsc::Sender};

use rand::random;

use super::{node::Node, node_flex::NodeFlex, trusted::Trusted};

pub struct Simulator {
    config: Config,
    nodes: Vec<NodeFlex>,
}

pub struct Config {
    // How many nodes in total come and go.
    pub nodes_flex: usize,
    // The probability (0..2**16-1) for an offline flex-node to go online after a tick.
    pub p_flex_sign_in: u16,
    // The probability (0..2**16-1) for an online flex-node to go offline after a tick.
    pub p_flex_sign_out: u16,
}

impl Config {
    pub fn default() -> Self {
        Self {
            nodes_flex: 10,
            p_flex_sign_in: 0x1000,
            p_flex_sign_out: 0x800,
        }
    }
}

impl Simulator {
    /// TODO: probably this'll need a Trusted tx-channel later on.
    pub fn new(config: Config, trusted: Sender<Trusted>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            nodes: (0..config.nodes_flex)
                .map(|_| NodeFlex::random(&trusted, config.p_flex_sign_out))
                .collect(),
            config,
        })
    }

    pub fn tick(&mut self) -> Result<Vec<Box<dyn Node>>, Box<dyn Error>> {
        let mut nodes = vec![];
        for node in self.nodes.iter_mut().filter(|n| !n.is_online()) {
            if random::<u16>() < self.config.p_flex_sign_in {
                nodes.push(node.go_online()?);
            }
        }
        Ok(nodes)
    }

    pub fn nodes_online(&self) -> usize {
        self.nodes.iter().filter(|v| v.is_online()).count()
    }
}

#[cfg(test)]
mod test {
    // use std::cmp::{max, min};

    // use super::*;

    // #[test]
    // fn test_online() {
    //     // let mut simul = Simulator::new(Config::default());
    //     // assert_eq!(simul.config.nodes_root, simul.nodes_online());

    //     // // Make sure that the number of nodes fluctuates somehow.
    //     // while simul.nodes_online() < 10 {
    //     //     simul.tick(0);
    //     // }
    //     // let (mut min_nodes, mut max_nodes) = (simul.config.nodes_root + simul.config.nodes_flex, 0);
    //     // for i in 1..100 {
    //     //     simul.tick(i);
    //     //     min_nodes = min(min_nodes, simul.nodes_online());
    //     //     max_nodes = max(max_nodes, simul.nodes_online());
    //     // }
    //     // assert!(min_nodes < 10);
    //     // assert!(max_nodes > 10);
    // }
}
