use assassin::broker::Broker;
use assassin::order::Order;
use assassin::position::Position;
use assassin::quote::Quote;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

extern crate greenback;
use greenback::Greenback as Money;

static TICKER: &'static str = "AAPL";
static DAYS_OUT: i32 = 45;

pub fn n_strikes_above(quotes: Vec<&Quote>, strikes: i32, price: Money) -> Option<&Quote> {
    if strikes < 1 {
        panic!("strikes must be > 0 (got: {})", strikes);
    }

    let mut res: Option<&Quote> = None;

    for q in &quotes {
        if q.strike_price() > price {
            res = Some(q);
            break;
        }
    }

    res
}

pub fn n_strikes_below(quotes: Vec<&Quote>, strikes: i32, price: Money) -> Option<&Quote> {
    if strikes < 1 {
        panic!("strikes must be > 0 (got: {})", strikes);
    }

    let mut res: Option<&Quote> = None;

    for q in &quotes {
        if q.strike_price() < price {
            res = Some(q);
        } else {
            break;
        }
    }

    res
}

pub struct PMCC {}

impl PMCC {
    pub fn new() -> PMCC {
        PMCC {}
    }

    // --------------------------------------------------------------------------------------------

    fn look_for_new_position_to_open(&self, broker: &Broker) {
        let quotes = broker.nearest_quotes_expiring_after_n_days(DAYS_OUT);

        let candidate = n_strikes_above(quotes, 2, broker.underlying_price_for(TICKER));
    }

    fn manage_positions(&self, broker: &Broker, positions: Vec<&Position>) {
        // if let Some(order) = self.generate_open_order(broker) {
        //     broker.process_order(order); // TODO: check result
        // }

        // if let Some(order) = self.generate_close_order(broker) {
        //     broker.process_order(order); // TODO: check result
        // }
    }

    // --------------------------------------------------------------------------------------------

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

    fn show_header(&self, broker: &Broker) {
        println!(
            "===== start of {} ==================================================",
            broker.current_date()
        );
        println!("");
    }

    fn show_summary(&self, broker: &Broker) {
        let current_date = broker.current_date();
        let day = current_date.format("%Y-%m-%d").to_string();

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
                position.expiration_date().num_days_from_ce()
                    - broker.current_date().num_days_from_ce(),
            );
            println!("format: {}", position.expiration_date().format("%Y-%m-%d"));
        }
        println!("");
    }
}

impl Model for PMCC {
    fn name(&self) -> &'static str {
        "Poor Man's Covered Call"
    }

    fn before_simulation(&mut self, _broker: &mut Broker) {}

    fn run_logic(&mut self, broker: &mut Broker) {
        self.show_header(broker);

        let positions = broker.open_positions();

        if positions.len() > 0 {
            self.manage_positions(broker, positions);
        } else {
            self.look_for_new_position_to_open(broker);
        }

        self.show_summary(broker);
    }

    fn after_simulation(&mut self, broker: &mut Broker) {
        // run again to handle the last day's data since
        // we won't be notified of it by the broker
        self.run_logic(broker);
    }
}
