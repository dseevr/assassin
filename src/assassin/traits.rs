use assassin::order::Order;
use assassin::position::Position;
use assassin::tick::Tick;

pub trait Broker {
	fn account_balance(&self) -> f64;
	fn process_order(&mut self, Order);
	fn open_positions(&self) -> Vec<Box<Position>>;
	fn total_trade_count(&self) -> i32;
}

pub trait DataFeed {
	fn next_tick(&mut self) -> Option<Tick>;
}

pub trait Model {
	fn get_name(&self) -> &'static str; // TODO: rename to just name()
	fn process_tick(&mut self, Tick, &mut Broker);
	fn before_simulation(&mut self, &mut Broker);
	fn after_simulation(&mut self, &mut Broker);
}
