use std::time::Instant;

use assassin::traits::*;

pub struct Simulation {
	feed: Box<DataFeed>,
	model: Box<Model>,
	// TODO: add settings variables (slippage, spread multipliers, etc.)
	// TODO: add target stats that the model must hit (sharpe, DD, etc.)

	ticks_processed: i64,

	start_time: Instant,
}

impl Simulation {
	pub fn new(model: Box<Model>, feed: Box<DataFeed>) -> Simulation {
		Simulation {
			feed: feed,
			model: model,
			ticks_processed: 0,
			start_time: Instant::now(),
		}
	}

	pub fn run(&mut self) {
		println!("running simulation");

		while let Some(tick) = self.feed.next_tick() {
			self.model.process_tick(tick);
			self.ticks_processed += 1;
		}

		println!("simulation finished");
		println!("processed {} ticks", self.ticks_processed);
	}

	pub fn total_run_time(&self) -> f64 {
		let seconds = self.start_time.elapsed().as_secs() as f64;
		let nanoseconds = self.start_time.elapsed().subsec_nanos() as f64 * 1e-9;

		seconds + nanoseconds
	}

        pub fn ticks_processed(&self) -> i64 {
                self.ticks_processed
        }
}
