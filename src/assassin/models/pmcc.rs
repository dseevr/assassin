use assassin::order::Order;
use assassin::quote::Quote;
use assassin::tick::Tick;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

pub struct PMCC {
	first_record: bool,
	current_date: i32,
}

impl PMCC {
	pub fn new() -> PMCC {
		PMCC{
			first_record: true,
			current_date: 0,
		}
	}

	fn generate_open_order(&self, quotes: &Vec<Quote>) -> Option<Order> {
		if quotes.is_empty() {
			return None;
		}

		// TODO: logic for picking a quote
		let quote = quotes[0].clone();

		let o = Order::new_buy_open_order(&quote, 10, 2.25);

		Some(o)
	}

	fn generate_close_order(&self, quotes: &Vec<Quote>) -> Option<Order> {
		// let o = Order::new_sell_close_order("AAPL".to_string(), 15.0, 10, 2.0);

		// Some(o)
		None
	}

	fn run_logic(&mut self, broker: &mut Broker) {
		let quotes = broker.quotes_for("AAPL".to_string());

		println!("running logic for day ({} records)", quotes.len());

		// TODO: update any charts, indicators, etc.
		// self.update_indicators(quotes);

		if let Some(order) = self.generate_open_order(&quotes) {
			broker.process_order(order); // TODO: check result
		}

		if let Some(order) = self.generate_close_order(&quotes) {
			broker.process_order(order); // TODO: check result
		}

		println!(
			"Cash at EOD: ${:.2} - positions open: {} - total orders: {} - commish paid: ${:.2}",
			broker.account_balance(),
			broker.open_positions().len(),
			broker.total_order_count(),
			broker.commission_paid(),
		);
	}
}

impl Model for PMCC {
	fn name(&self) -> &'static str {
		"Poor Man's Covered Call"
	}

	fn before_simulation(&mut self, _broker: &mut Broker) {}

	// NOTE: this is a hack to ensure that we only run_logic() once
	//       per day because we don't have intraday data.
	fn process_tick(&mut self, tick: Tick, broker: &mut Broker) {
		let current_date = tick.date().num_days_from_ce();

		if self.first_record {
			self.first_record = false;
			self.current_date = current_date;
			return;
		}

		// still gathering data for the current day
		if current_date == self.current_date {
			return;
		}

		// day has changed, so run normal logic
		self.run_logic(broker);

		// prepare for the next day
		self.current_date = current_date;
	}

	fn after_simulation(&mut self, broker: &mut Broker) {
		// run again to handle the last day's data
		self.run_logic(broker);
	}
}