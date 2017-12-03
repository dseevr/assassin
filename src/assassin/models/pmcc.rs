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

#[allow(dead_code)]
pub fn print_quote(q: &Quote, date: DateTime<Utc>) {
    let call = q.is_call();
    let strike = q.strike_price();
    let bid = q.bid();
    let ask = q.ask();
    let t = if call { "C" } else { "P" };
    let days = q.days_to_expiration(date);

    println!("{} {} {}/{} {} days left", t, strike, bid, ask, days);
}

#[allow(dead_code)]
pub fn print_chain(quotes: Vec<&Quote>, date: DateTime<Utc>) {
    for q in quotes {
        print_quote(q, date);
    }
}

pub fn n_strikes_above(quotes: Vec<&Quote>, strikes: i32, price: Money) -> Option<&Quote> {
    if strikes < 1 {
        panic!("strikes must be > 0 (got: {})", strikes);
    }

    let mut found = false;
    let mut count = 0;

    for q in &quotes {
        // if we have found the strike above the underlying...
        if found {
            count += 1;

            if count == strikes {
                return Some(q);
            } else {
                if count > strikes {
                    return None; // didn't find anything
                }
            }
        } else {
            if q.strike_price() > price {
                found = true;
                count = 1; // 1 strike above
            }
        }
    }

    None
}

pub fn n_strikes_below(quotes: Vec<&Quote>, strikes: i32, price: Money) -> Option<&Quote> {
    if strikes < 1 {
        panic!("strikes must be > 0 (got: {})", strikes);
    }

    let mut found = false;
    let mut count = 0;

    let mut reversed_quotes = quotes.clone();
    reversed_quotes.reverse();

    for q in &reversed_quotes {
        if found {
            count += 1;

            if count == strikes {
                return Some(q);
            } else {
                if count > strikes {
                    return None;
                }
            }
        } else {
            if q.strike_price() < price {
                found = true;
                count = 1;
            }
        }
    }

    None
}

static SHORT_DAYS_OUT_MIN: i32 = 30;
static SHORT_DAYS_OUT_MAX: i32 = 40;
static LONG_DAYS_OUT_MIN: i32 = 150;
static LONG_DAYS_OUT_MAX: i32 = 200;
static NUM_CONTRACTS: i32 = 5;
static STRIKES_ABOVE: i32 = 2;
static STRIKES_BELOW: i32 = 4;

pub struct PMCC {}

impl PMCC {
    pub fn new() -> PMCC {
        PMCC {}
    }

    // --------------------------------------------------------------------------------------------

    fn look_for_new_short_position_to_open(&self, broker: &mut Broker) -> Option<Order> {
        let underlying_price = broker.underlying_price_for(TICKER);

        // println!(
        //     "** Searching for candidate quote for upper call ({} strikes below)",
        //     STRIKES_BELOW
        // );

        let quotes: Vec<&Quote> = broker
            .nearest_quotes_expiring_between_n_days(SHORT_DAYS_OUT_MIN, SHORT_DAYS_OUT_MAX)
            .into_iter()
            .filter(|q| q.is_call())
            .collect();

        // print_chain(quotes.clone(), date);

        let quote = match n_strikes_above(quotes.clone(), STRIKES_ABOVE, underlying_price) {
            Some(quote) => {
                // println!("** Found candidate:");
                // print_quote(quote, date);
                quote
            }
            None => {
                // println!("!! No quote found");
                return None;
            }
        };

        let o = Order::new_sell_open_order(quote, NUM_CONTRACTS, quote.midpoint_price());

        Some(o)
    }

    fn look_for_new_long_position_to_open(&self, broker: &mut Broker) -> Option<Order> {
        let underlying_price = broker.underlying_price_for(TICKER);

        // println!(
        //     "** Searching for candidate quote for lower call ({} strikes below)",
        //     STRIKES_BELOW
        // );

        let quotes: Vec<&Quote> = broker
            .nearest_quotes_expiring_between_n_days(LONG_DAYS_OUT_MIN, LONG_DAYS_OUT_MAX)
            .into_iter()
            .filter(|q| q.is_call())
            .collect();

        // print_chain(quotes.clone(), date);

        let quote = match n_strikes_below(quotes, STRIKES_BELOW, underlying_price) {
            Some(quote) => {
                // println!("** Found candidate:");
                // print_quote(quote, date);
                quote
            }
            None => {
                // println!("!! No quote found");
                return None;
            }
        };

        let o = Order::new_buy_open_order(quote, NUM_CONTRACTS, quote.midpoint_price());

        Some(o)
    }


    fn manage_positions(&self, _broker: &mut Broker, _positions: Vec<Position>) -> Vec<Order> {
        vec![]
    }
}

impl Model for PMCC {
    fn name(&self) -> &'static str {
        "Poor Man's Covered Call"
    }

    fn before_simulation(&mut self, _broker: &mut Broker) {}

    fn run_logic(&mut self, broker: &mut Broker) {
        let positions = broker.open_positions();

        let mut orders = vec![];

        match positions.len() {
            2 => {
                // println!("** Managing existing positions");
                orders = self.manage_positions(broker, positions);

                for o in orders {
                    broker.process_order(o);
                }
            }
            1 => {
                if positions[0].is_long() {
                    // println!("** Opening new short position");
                    if let Some(o) = self.look_for_new_short_position_to_open(broker) {
                        orders.push(o);
                    }
                } else {
                    if let Some(o) = self.look_for_new_long_position_to_open(broker) {
                        orders.push(o);
                    }
                }

                for o in orders {
                    broker.process_order(o);
                }
            }
            0 => {
                // println!("** Looking for new positions to open");
                if let Some(o) = self.look_for_new_short_position_to_open(broker) {
                    orders.push(o);
                }

                if let Some(o) = self.look_for_new_long_position_to_open(broker) {
                    orders.push(o);
                }

                // only process orders if we found candidates for both
                if orders.len() == 2 {
                    for o in orders {
                        broker.process_order(o);
                    }
                }
            }
            _ => panic!("unexpected number of positions: {}", positions.len()),
        };
    }

    fn after_simulation(&mut self, broker: &mut Broker) {
        // run again to handle the last day's data since
        // we won't be notified of it by the broker
        self.run_logic(broker);
    }

    fn show_bod_header(&self, broker: &Broker) {
        println!(
            "===== start of {} ======= Balance: {} =================",
            broker.current_date(),
            broker.account_balance(),
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
