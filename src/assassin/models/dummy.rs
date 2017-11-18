use assassin::tick::Tick;
use assassin::traits::*;

pub struct DummyModel {}

impl DummyModel {
	pub fn new() -> DummyModel {
		DummyModel{}
	}
}

impl Model for DummyModel {
	fn get_name(&self) -> &'static str {
		"dummy model"
	}

	fn process_tick(&mut self, tick: Tick) {
		// if tick.volume < 1000 {
		// 	return;
		// }

		// if tick.intrinsic_value() < 10.0 {
		// 	return;
		// }

		tick.print_deets();
	}
}