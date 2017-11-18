use assassin::tick::Tick;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

pub struct PMCC {
	first_record: bool,
	current_date: i32,
	ticks: Vec<Box<Tick>>,
}

impl PMCC {
	pub fn new() -> PMCC {
		PMCC{
			first_record: true,
			current_date: 0,
			ticks: vec![],
		}
	}

	fn run_logic(&mut self) {
		println!("running logic for day ({} records)", self.ticks.len());
	}
}

impl Model for PMCC {
	fn get_name(&self) -> &'static str {
		"Poor Man's Covered Call"
	}

	fn before_simulation(&mut self) {}

	fn process_tick(&mut self, tick: Tick) {
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
		self.run_logic();

		// prepare for the next day
		self.ticks.clear();
		self.ticks.push(Box::new(tick));
		self.current_date = current_date;
	}

	fn after_simulation(&mut self) {
		// run again to handle the last day's data
		self.run_logic();
	}
}