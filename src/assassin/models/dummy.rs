use assassin::tick::Tick;
use assassin::traits::*;

pub struct DummyModel {}

impl DummyModel {
	pub fn new() -> DummyModel {
		DummyModel{}
	}
}

impl Model for DummyModel {
	fn name(&self) -> &'static str {
		"dummy model"
	}

	fn before_simulation(&mut self) {}

	fn process_tick(&mut self, _tick: Tick) {}

	fn after_simulation(&mut self) {}
}