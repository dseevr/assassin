use assassin::broker::Broker;
use assassin::order::Order;
use assassin::quote::Quote;

extern crate greenback;
use greenback::Greenback as Money;

pub trait Commission {
    fn commission_for(&self, &Order) -> Money;
}

pub trait DataFeed {
    fn next_quote(&mut self) -> Option<Quote>;
}

pub trait Model {
    fn name(&self) -> &'static str;
    fn before_simulation(&mut self, &mut Broker);
    fn after_simulation(&mut self, &mut Broker);
    fn run_logic(&mut self, &mut Broker);
    fn show_bod_header(&self, &Broker);
    fn show_eod_summary(&self, &Broker);
}
