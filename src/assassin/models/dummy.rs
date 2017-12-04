use assassin::broker::Broker;
use assassin::order::Order;
use assassin::traits::*;

#[allow(dead_code)]
pub struct DummyModel {}

#[allow(dead_code)]
impl DummyModel {
    pub fn new() -> DummyModel {
        DummyModel {}
    }
}

impl Model for DummyModel {
    fn name(&self) -> &'static str {
        "dummy model"
    }

    fn before_simulation(&mut self, _b: &Broker) {}
    fn after_simulation(&mut self, _b: &Broker) {}
    fn run_logic(&mut self, _b: &Broker) -> Vec<Order> {
        vec![]
    }
    fn show_bod_header(&self, _b: &Broker) {}
    fn show_eod_summary(&self, _b: &Broker) {}
}
