use assassin::order::Order;
use assassin::tick::Tick;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

pub struct PMCC {
	first_record: bool,
	current_date: i32,
	ticks: Vec<Tick>,
}

impl PMCC {
	pub fn new() -> PMCC {
		PMCC{
			first_record: true,
			current_date: 0,
			ticks: vec![],
		}
	}

	fn generate_open_order(&self) -> Option<Order> {
		let o = Order::new_buy_open_order("AAPL".to_string(), 15.0, 10, 2.25);

		Some(o)
	}

	fn generate_close_order(&self) -> Option<Order> {
		// let o = Order::new_sell_close_order("AAPL".to_string(), 15.0, 10, 2.0);

		// Some(o)
		None
	}

	fn run_logic(&mut self, broker: &mut Broker) {
		println!("running logic for day ({} records)", self.ticks.len());

		// for _tick in &self.ticks {
		// 	// self.update_indicators(tick);
		// }

		// at EOD, see if we should buy or sell anything

		if let Some(order) = self.generate_open_order() {
			broker.process_order(order); // TODO: check result
		}

		if let Some(order) = self.generate_close_order() {
			broker.process_order(order); // TODO: check result
		}

		println!(
			"Cash at EOD: ${:.2} - positions open: {} - total orders: {}",
			broker.account_balance(),
			broker.open_positions().len(),
			broker.total_order_count(),
		);
	}
}

impl Model for PMCC {
	fn name(&self) -> &'static str {
		"Poor Man's Covered Call"
	}

	fn before_simulation(&mut self, _broker: &mut Broker) {}

	fn process_tick(&mut self, tick: Tick, broker: &mut Broker) {
		let current_date = tick.date().num_days_from_ce();

		if self.first_record {
			self.first_record = false;
			self.current_date = current_date;
		}

		// still gathering data for the current day
		if current_date == self.current_date {
			self.ticks.push(tick);
			return;
		}

		// day has changed, so run normal logic
		self.run_logic(broker);

		// prepare for the next day
		self.ticks.clear();
		self.ticks.push(tick);
		self.current_date = current_date;
	}

	fn after_simulation(&mut self, broker: &mut Broker) {
		// run again to handle the last day's data
		self.run_logic(broker);
	}
}