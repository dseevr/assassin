use assassin::tick::Tick;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

pub struct PMCC {
	first_record: bool,
	current_date: i32,
	ticks: Vec<Box<Tick>>,
}

pub struct Order {

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
		None
	}

	fn generate_close_order(&self) -> Option<Order> {
		None
	}

	fn run_logic(&mut self, broker: &Broker) {
		println!("running logic for day ({} records)", self.ticks.len());

		for tick in &self.ticks {
			// self.update_indicators(tick);

			if let Some(order) = self.generate_open_order() {
				// broker.process_order(order)
			}

			if let Some(order) = self.generate_close_order() {
				// broker.process_order(order)
			}
		}

		println!("Cash at EOD: ${:.2}", broker.account_balance());
	}
}

impl Model for PMCC {
	fn get_name(&self) -> &'static str {
		"Poor Man's Covered Call"
	}

	fn before_simulation(&mut self, _broker: &Broker) {}

	fn process_tick(&mut self, tick: Tick, broker: &Broker) {
		let current_date = tick.date().num_days_from_ce();

		if self.first_record {
			self.first_record = false;
			self.current_date = current_date;
		}

		// still gathering data for the current day
		if current_date == self.current_date {
			self.ticks.push(Box::new(tick));
			return;
		}

		// day has changed, so run normal logic
		self.run_logic(broker);

		// prepare for the next day
		self.ticks.clear();
		self.ticks.push(Box::new(tick));
		self.current_date = current_date;
	}

	fn after_simulation(&mut self, broker: &Broker) {
		// run again to handle the last day's data
		self.run_logic(broker);
	}
}