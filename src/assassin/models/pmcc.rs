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

pub fn print_chain(quotes: Vec<&Quote>, date: DateTime<Utc>) {
    for q in quotes {
        let call = q.is_call();
        let strike = q.strike_price();
        let bid = q.bid();
        let ask = q.ask();
        let t = if call { "C" } else { "P" };
        let days = q.days_to_expiration(date);

        println!("{} {} {}/{} {} days left", t, strike, bid, ask, days);
    }
}

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

static DAYS_OUT_MIN: i32 = 30;
static DAYS_OUT_MAX: i32 = 40;

pub struct PMCC {}

impl PMCC {
    pub fn new() -> PMCC {
        PMCC {}
    }

    // --------------------------------------------------------------------------------------------

    fn look_for_new_position_to_open(&self, broker: &mut Broker) -> Vec<Order> {
        let mut res = vec![];

        println!("** Searching for candidate quote for upper call");

        let quotes: Vec<&Quote> = broker
            .nearest_quotes_expiring_between_n_days(DAYS_OUT_MIN, DAYS_OUT_MAX)
            .into_iter()
            .filter(|q| q.is_call())
            .collect();

        print_chain(quotes.clone(), broker.current_date());

        // TODO: get rid of clone
        let quote = match n_strikes_above(quotes.clone(), 1, broker.underlying_price_for(TICKER)) {
            Some(quote) => quote,
            None => {
                println!("!! No quote found");
                return vec![];
            }
        };

        let o = Order::new_sell_open_order(quote, 10, quote.midpoint_price());
        res.push(o);

        println!("** Searching for candidate quote for lower call");

        let quote = match n_strikes_below(quotes, 3, broker.underlying_price_for(TICKER)) {
            Some(quote) => quote,
            None => {
                println!("!! No quote found");
                return vec![];
            }
        };

        let o = Order::new_buy_open_order(quote, 10, quote.midpoint_price());
        res.push(o);

        res
    }

    fn manage_positions(&self, broker: &mut Broker, positions: Vec<Position>) -> Vec<Order> {
        vec![]
    }
}

impl Model for PMCC {
    fn name(&self) -> &'static str {
        "Poor Man's Covered Call"
    }

    fn before_simulation(&mut self, _broker: &mut Broker) {}

    fn run_logic(&mut self, broker: &mut Broker) {
        let positions = broker.positions();

        let orders = if positions.len() > 0 {
            println!("** Managing existing postitions");
            self.manage_positions(broker, positions)
        } else {
            println!("** Looking for new positions to open");
            self.look_for_new_position_to_open(broker)
        };

        for o in orders {
            broker.process_order(o);
        }
    }

    fn after_simulation(&mut self, broker: &mut Broker) {
        // run again to handle the last day's data since
        // we won't be notified of it by the broker
        self.run_logic(broker);
    }

    fn show_bod_header(&self, broker: &Broker) {
        println!(
            "===== start of {} ==================================================",
            broker.current_date()
        );
        println!("");
    }

    fn show_eod_summary(&self, broker: &Broker) {
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
