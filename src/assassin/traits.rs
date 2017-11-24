use assassin::broker::Broker;
use assassin::money::Money;
use assassin::order::Order;
use assassin::tick::Tick;

pub trait Commission {
	fn commission_for(&self, &Order) -> Money;
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
