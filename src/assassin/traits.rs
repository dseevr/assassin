use assassin::order::Order;
use assassin::position::Position;
use assassin::quote::Quote;
use assassin::tick::Tick;

pub trait Broker {
	fn account_balance(&self) -> f64;
	fn process_order(&mut self, Order) -> bool;
	fn open_positions(&self) -> Vec<Position>;
	fn total_order_count(&self) -> i32;
	fn commission_paid(&self) -> f64;
	fn close_all_positions(&mut self);
	fn next_tick(&mut self) -> Option<Tick>;
	fn quotes_for(&self, String) -> Vec<Quote>;
}

pub trait Commission {
	fn commission_for(&self, &Order) -> f64;
}

pub trait DataFeed {
	fn next_tick(&mut self) -> Option<Tick>;
}

pub trait Model {
	fn name(&self) -> &'static str;
	fn before_simulation(&mut self, &mut Broker);
	fn after_simulation(&mut self, &mut Broker);

	// TODO: rename to run_logic()
	//       broker can wake up the model when it's ready for a decision to be made.
	//       this could be daily with EOD data or after every tick or 5 seconds, etc.
	//       when running against realtime data.
	fn process_tick(&mut self, Tick, &mut Broker);
}
