use std::time::Instant;

use assassin::traits::*;

pub struct Simulation {
	model: Box<Model>,
	broker: Box<Broker>,

	// TODO: add settings variables (slippage, spread multipliers, etc.)
	// TODO: add target stats that the model must hit (sharpe, DD, etc.)

	ticks_processed: i64,

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
			ticks_processed: 0,
			start_time: Instant::now(),
			starting_balance: starting_balance,
		}
	}

	pub fn run(&mut self) {
		self.model.before_simulation(&mut *self.broker);

		while let Some(tick) = self.broker.next_tick() {
			// TODO: maybe check that the ticks are in chronological order here?
			// TODO: how should the broker notify the model that it's ready for
			//       a buy/sell decision, and how should the model actually apply
			//       that on the broker without a dependency loop?
			self.model.process_tick(tick, &mut *self.broker);
			self.ticks_processed += 1;
		}

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

		let growth = if self.starting_balance > balance {
			-((1.0 - (balance / self.starting_balance)) * 100.0)
		} else if balance > self.starting_balance {
			1.0 - ((self.starting_balance / balance) * 100.0)
		} else {
			0.0 // no orders placed so 0% growth
		};

		println!("Capital growth: {:.2}%", growth);
		println!("Total orders: {}", self.broker.total_order_count());

		println!("");
		println!(
			"Ran simulation ({} ticks) in {:.2} seconds",
			self.ticks_processed,
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
