use super::msgs::NetworkMsg;

pub trait RxTx {
    fn receive(&mut self, input: Vec<NetworkMsg>) -> Vec<NetworkMsg>;
    fn tick(&mut self, time: u64) -> Vec<NetworkMsg>;
}
