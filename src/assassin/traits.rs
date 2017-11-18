use assassin::tick::Tick;

struct Position {

}

pub trait Broker {
	fn account_balance(&self) -> f64;
	fn process_order(&mut self);
	// fn open_positions(&self) -> Vec<Box<Position>>;
}

pub trait DataFeed {
	fn next_tick(&mut self) -> Option<Tick>;
}

pub trait Model {
	fn get_name(&self) -> &'static str; // TODO: rename to just name()
	fn process_tick(&mut self, Tick, &Broker);
	fn before_simulation(&mut self, &Broker);
	fn after_simulation(&mut self, &Broker);
}
