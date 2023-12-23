use std::sync::mpsc::Sender;

use tracing::{debug, error, trace};

use crate::simul::{
    broker::BMNet,
    node::{Node, NodeInfo},
    trusted::TReqMsg,
};

use super::{
    broker::{BMWeb, BrokerMsg},
    trusted::{TrustedReply, TrustedRequest},
};

pub struct Web {
    trusted: Sender<TrustedRequest>,
}

impl Web {
    pub fn new(trusted: Sender<TrustedRequest>) -> Self {
        Self { trusted }
    }

    pub fn action(&mut self, action: BMWeb) -> Vec<BrokerMsg> {
        match action {
            BMWeb::WebRegister(secret) => {
                let id = Node::secret_to_id(secret);
                match TReqMsg::Info(id).send(&self.trusted) {
                    Ok(reply) => {
                        if let TrustedReply::NodeInfo(info_op) = reply {
                            let info = info_op.unwrap_or_else(|| {
                                debug!("Creating new node with id {id:#034x}");
                                NodeInfo::with_id(id)
                            });
                            return vec![BrokerMsg::Network(BMNet::NodeAdd(Node::from_info(
                                info,
                                &self.trusted,
                            )))];
                        }
                    }
                    Err(e) => {
                        error!("While sending to trusted: {e:?}");
                    }
                }
            }
        }
        vec![]
    }

    pub fn tick(&mut self, time: u64) -> Vec<BrokerMsg> {
        trace!("Tick @ {time}");
        vec![]
    }
}
