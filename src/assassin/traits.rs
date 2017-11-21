use assassin::order::Order;
use assassin::position::Position;
use assassin::quote::Quote;
use assassin::tick::Tick;

extern crate chrono;
use self::chrono::prelude::*;

pub trait Broker {
	fn account_balance(&self) -> f64;
	fn process_order(&mut self, Order) -> bool;
	fn open_positions(&self) -> Vec<Position>;
	fn positions(&self) -> Vec<Position>;
	fn total_order_count(&self) -> i32;
	fn commission_paid(&self) -> f64;
	fn close_all_positions(&mut self);
	fn quote_for(&self, String) -> Option<Quote>;
	fn quotes_for(&self, String) -> Vec<Quote>;
	fn current_date(&self) -> DateTime<FixedOffset>;
	fn process_simulation_data(&mut self, &mut Model);
	fn ticks_processed(&self) -> i64;
	fn close_expired_positions(&mut self);
}

pub trait Commission {
	fn commission_for(&self, &Order) -> f64;
}

pub trait DataFeed {
	// TODO: have a way to detect if the data we're parsing
	//       is incorrectly containing duplicated data for holidays
	fn next_tick(&mut self) -> Option<Tick>;
}

pub trait Model {
	fn name(&self) -> &'static str;
	fn before_simulation(&mut self, &mut Broker);
	fn after_simulation(&mut self, &mut Broker);
	fn run_logic(&mut self, &mut Broker);
}
