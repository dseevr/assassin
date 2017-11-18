use assassin::order::Order;
use assassin::position::Position;
use assassin::tick::Tick;

pub trait Broker {
	fn account_balance(&self) -> f64;
	fn process_order(&mut self, Order) -> bool;
	fn open_positions(&self) -> Vec<Position>;
	fn total_order_count(&self) -> i32;
	fn commission_paid(&self) -> f64;
	fn close_all_positions(&mut self);
	fn next_tick(&mut self) -> Option<Tick>;
}

pub trait Commission {
	fn commission_for(&self, Order) -> f64;
}

pub trait DataFeed {
	fn next_tick(&mut self) -> Option<Tick>;
}

pub trait Model {
	fn name(&self) -> &'static str;
	fn process_tick(&mut self, Tick, &mut Broker);
	fn before_simulation(&mut self, &mut Broker);
	fn after_simulation(&mut self, &mut Broker);
}
