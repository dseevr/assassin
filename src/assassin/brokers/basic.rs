use assassin::traits::*;

pub struct BasicBroker {

}

impl BasicBroker {
	pub fn new() -> BasicBroker {
		BasicBroker{

		}
	}
}

impl Broker for BasicBroker {
	fn account_balance(&self) -> f64 {
		0.0
	}

	fn process_order(&mut self) {
		
	}
}