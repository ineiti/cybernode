use std::sync::mpsc::Sender;

use tracing::error;

use crate::simul::{node::Node, trusted::{TReqMsg, NodeInfo}};

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
                        println!("Reply is: {reply:?}");
                        if let TrustedReply::NodeInfo(info_op) = reply {
                            let info = info_op.unwrap_or_else(||{
                                NodeInfo::random()
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
        todo!()
    }
}
