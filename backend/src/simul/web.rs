use super::broker::Module;

pub struct Web {}

impl Web {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module for Web {
    fn action(&mut self, action: super::broker::BrokerAction) -> Vec<super::broker::BrokerAction> {
        todo!()
    }

    fn tick(&mut self, time: u64) -> Vec<super::broker::BrokerAction> {
        todo!()
    }
}
