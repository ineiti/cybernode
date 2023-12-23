use std::sync::mpsc::Sender;

use tracing::{debug, error, trace};

use crate::simul::{node::{Node, NodeInfo}, trusted::TReqMsg};

use super::{
    broker::{BrokerAction, Module},
    trusted::{TrustedReply, TrustedRequest},
};

pub struct Web {
    trusted: Sender<TrustedRequest>,
}

impl Web {
    pub fn new(trusted: Sender<TrustedRequest>) -> Self {
        Self { trusted }
    }
}

impl Module for Web {
    fn action(&mut self, action: super::broker::BrokerAction) -> Vec<BrokerAction> {
        match action {
            super::broker::BrokerAction::WebRegister(secret) => {
                let id = Node::secret_to_id(secret);
                match TReqMsg::Info(id).send(&self.trusted) {
                    Ok(reply) => {
                        if let TrustedReply::NodeInfo(info_op) = reply {
                            let info = info_op.unwrap_or_else(|| {
                                debug!("Creating new node with id {id:#034x}");
                                NodeInfo {
                                    id,
                                    last_seen: 0,
                                    mana: 0.into(),
                                }
                            });
                            return vec![BrokerAction::NodeAdd(Node::from_info(
                                info,
                                &self.trusted,
                                true,
                            ))];
                        }
                    }
                    Err(e) => {
                        error!("While sending to trusted: {e:?}");
                    }
                }
            }
            _ => {}
        }
        vec![]
    }

    fn tick(&mut self, time: u64) -> Vec<super::broker::BrokerAction> {
        trace!("Tick @ {time}");
        vec![]
    }
}
