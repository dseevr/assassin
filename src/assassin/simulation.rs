use std::time::Instant;

use assassin::position::Position;

use assassin::traits::*;

pub struct Simulation {
	model: Box<Model>,
	broker: Box<Broker>,

	// TODO: add settings variables (slippage, spread multipliers, etc.)
	// TODO: add target stats that the model must hit (sharpe, DD, etc.)

	start_time: Instant,
	starting_balance: f64,
}

impl Simulation {
	pub fn new(
		model: Box<Model>,
		broker: Box<Broker>,
		) -> Simulation {

		let starting_balance = broker.account_balance();

		Simulation {
			model: model,
			broker: broker,
			start_time: Instant::now(),
			starting_balance: starting_balance,
		}
	}

	pub fn run(&mut self) {
		self.model.before_simulation(&mut *self.broker);

		// TODO: broker and model should communicate via a channel
		self.broker.process_simulation_data(&mut *self.model);

		self.model.after_simulation(&mut *self.broker);

		// do this after after_simulation to allow for EOD data to have a chance
		// to do something on the last day of data
		self.broker.close_all_positions();
	}

	pub fn print_stats(&self) {

		let balance = self.broker.account_balance();

		println!("===== RESULTS =====");
		println!("");
		println!("Starting balance: ${:.2}", self.starting_balance);
		println!("Ending balance: ${:.2}", balance);

		let growth = ((balance / self.starting_balance) * 100.0) - 100.0;

		println!("Capital growth: {:.2}%", growth);
		println!("Total orders: {}", self.broker.total_order_count());
		println!("");

		println!("===== POSITIONS =====");
		println!("");

		let mut running_total = self.starting_balance;

		let mut positions = self.broker.positions().clone();
		positions.sort_by(|a,b| a.name().cmp(&b.name()));

		for pos in positions {
			println!("----- {} -----", pos.name());

			for o in pos.orders() {
				running_total += o.canonical_cost_basis();

				// BUY 10 contracts @ $15
				println!(
					"  {} {} {} contracts @ ${:.2}",
					o.buy_or_sell_string(),
					o.quantity(),
					o.option_name(),
					o.fill_price(),
				);
			}
			println!("");

			println!("Commission paid: ${:.2}", pos.commission_paid());
			println!("Position value: ${:.2}", pos.realized_profit());
			println!("Running total: ${:.2}", running_total);
			println!("");
		}

		println!("");
		println!(
			"Ran simulation ({} ticks) in {:.2} seconds",
			self.broker.ticks_processed(),
			self.total_run_time(),
		);
		println!("");
	}

	pub fn total_run_time(&self) -> f64 {
		let seconds = self.start_time.elapsed().as_secs() as f64;
		let nanoseconds = self.start_time.elapsed().subsec_nanos() as f64 * 1e-9;

		seconds + nanoseconds
	}
}
