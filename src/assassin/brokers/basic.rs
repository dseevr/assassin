use assassin::order::Order;
use assassin::position::Position;
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

	fn process_order(&mut self, order: Order) {

	}

	fn open_positions(&self) -> Vec<Box<Position>> {
		vec![]
	}

	fn total_trade_count(&self) -> i32 {
		0
	}
}