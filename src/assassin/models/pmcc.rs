use assassin::broker::Broker;
use assassin::order::Order;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

static TICKER: &'static str = "AAPL";

pub struct PMCC {}

impl PMCC {
    pub fn new() -> PMCC {
        PMCC {}
    }

    fn generate_open_order(&self, broker: &mut Broker) -> Option<Order> {
        if broker.open_positions().len() >= 5 {
            return None;
        }

        let quote = &broker.call_quotes_for(TICKER)[0];

        let o = Order::new_buy_open_order(quote, 10, quote.midpoint_price());

        Some(o)
    }

    fn generate_close_order(&self, _broker: &mut Broker) -> Option<Order> {
        // let o = Order::new_sell_close_order("AAPL".to_string(), 15.0, 100, 2.0);

        // Some(o)
        None
    }
}

impl Model for PMCC {
    fn name(&self) -> &'static str {
        "Poor Man's Covered Call"
    }

    fn before_simulation(&mut self, _broker: &mut Broker) {}

    fn run_logic(&mut self, broker: &mut Broker) {
        let current_date = broker.current_date();
        let day = current_date.format("%Y-%m-%d");

        println!(
            "===== start of {} ==================================================",
            day
        );
        println!("");

        // TODO: update any charts, indicators, etc.
        // self.update_indicators(quotes);

        if let Some(order) = self.generate_open_order(broker) {
            broker.process_order(order); // TODO: check result
        }

        if let Some(order) = self.generate_close_order(broker) {
            broker.process_order(order); // TODO: check result
        }

        // show summary for day
        println!("");
        println!(" ----- {} end of day summary -----", day);
        println!("");
        println!(
            "Balance: {}\npositions open: {}\ntotal orders: {}\ncommish paid: {}",
            broker.account_balance(),
            broker.open_positions().len(),
            broker.total_order_count(),
            broker.commission_paid(),
        );
        println!("");

        println!("Positions:");
        for position in broker.open_positions() {
            println!(
                "{} - {} contracts - Expires: {} days",
                position.symbol(),
                position.quantity(),
                position.expiration_date().num_days_from_ce() - current_date.num_days_from_ce(),
            );
            println!("format: {}", position.expiration_date().format("%Y-%m-%d"));
        }
        println!("");
    }

    fn after_simulation(&mut self, broker: &mut Broker) {
        // run again to handle the last day's data since
        // we won't be notified of it by the broker
        self.run_logic(broker);
    }
}
