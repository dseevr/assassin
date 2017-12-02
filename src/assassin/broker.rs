use std::rc::Rc;

use assassin::order::Order;
use assassin::position::Position;
use assassin::quote::Quote;
use assassin::traits::*;

extern crate chrono;
use self::chrono::prelude::*;

extern crate fnv;
use self::fnv::FnvHashMap;

extern crate greenback;
use greenback::Greenback as Money;

pub struct Broker {
    balance: Money,
    positions: FnvHashMap<Rc<str>, Position>,
    orders: Vec<Order>,
    commission_schedule: Box<Commission>,
    commission_paid: Money,
    data_feed: Box<DataFeed>,
    // TODO: convert this into a FnvHashMap<Rc<str>, FnvHashMap<Rc<str>, Quote>>
    quotes: FnvHashMap<Rc<str>, Quote>,
    current_date: DateTime<Utc>,
    ticks_processed: i64,
    quote_map_capacity: usize,
    underlying_prices: FnvHashMap<Rc<str>, Money>,
}

impl Broker {
    pub fn new(
        initial_balance: Money,
        commission_schedule: Box<Commission>,
        data_feed: Box<DataFeed>,
    ) -> Broker {
        if initial_balance <= Money::zero() {
            panic!("balance must be > 0.0 (got {})", initial_balance);
        }

        // this is just so we have a default value
        let current_date = Utc::now();

        Broker {
            balance: initial_balance,
            positions: FnvHashMap::default(),
            orders: vec![],
            commission_schedule: commission_schedule,
            commission_paid: Money::zero(),
            data_feed: data_feed,
            quotes: FnvHashMap::with_capacity_and_hasher(0, Default::default()),
            current_date: current_date,
            ticks_processed: 0,
            quote_map_capacity: 0,
            underlying_prices: FnvHashMap::default(),
        }
    }

    #[allow(dead_code)]
    pub fn underlying_price_for(&self, symbol: &str) -> Money {
        *self.underlying_prices.get(symbol).unwrap()
    }

    // TODO: this should only return quotes for the desired symbol
    pub fn quotes_for(&self, _symbol: &str) -> Vec<&Quote> {
        self.quotes.iter().map(|(_, q)| q).collect()
    }

    pub fn nearest_quotes_expiring_after_n_days(&self, days: i32) -> Vec<&Quote> {
        if days < 0 {
            panic!("days must be >= 0 (got: {})", days);
        }

        let all_quotes: Vec<&Quote> = self.quotes.iter().map(|(_, q)| q).collect();

        let expiring_quotes: Vec<&Quote> = all_quotes
            .into_iter()
            .filter(|q| q.days_to_expiration(self.current_date()) > days)
            .collect();

        let days_to_expiration = expiring_quotes
            .first()
            .unwrap()
            .days_to_expiration(self.current_date);

        expiring_quotes
            .into_iter()
            .filter(|q| {
                q.days_to_expiration(self.current_date) < days_to_expiration + 1
            })
            .collect()
    }

    pub fn calls_expiring_after_n_days(&self, days: i32) -> Vec<&Quote> {
        self.nearest_quotes_expiring_after_n_days(days)
            .into_iter()
            .filter(|q| q.is_call())
            .collect()
    }

    pub fn puts_expiring_after_n_days(&self, days: i32) -> Vec<&Quote> {
        self.nearest_quotes_expiring_after_n_days(days)
            .into_iter()
            .filter(|q| q.is_put())
            .collect()
    }

    pub fn call_quotes_for(&self, symbol: &str) -> Vec<&Quote> {
        let mut quotes: Vec<&Quote> = self.quotes_for(symbol)
            .into_iter()
            .filter(|q| q.is_call())
            .collect();
        quotes.sort_by(|a, b| a.name().cmp(&b.name()));

        quotes
    }

    #[allow(dead_code)]
    pub fn put_quotes_for(&self, symbol: &str) -> Vec<&Quote> {
        let mut quotes: Vec<&Quote> = self.quotes_for(symbol)
            .into_iter()
            .filter(|q| q.is_put())
            .collect();
        quotes.sort_by(|a, b| a.name().cmp(&b.name()));

        quotes
    }

    pub fn process_simulation_data(&mut self, model: &mut Model) {
        let mut day_changed;

        // manually consume the first tick here so we don't have to check
        // to see if it's the first tick every single time
        {
            let first_tick = self.data_feed.next_tick().unwrap();
            self.current_date = first_tick.date();
            self.quotes
                .insert(first_tick.name(), Quote::new(&first_tick));
            self.underlying_prices
                .insert(first_tick.symbol(), first_tick.underlying_price());
        }

        while let Some(tick) = self.data_feed.next_tick() {
            day_changed = tick.date() != self.current_date;

            // ----- trading day logic -----------------------------------------

            // with EOD data, we run every time the day changes
            if day_changed {
                // TODO: convert this to a channel send with a timeout
                model.run_logic(self);
            }

            // ----- after hours cleanup ---------------------------------------

            self.underlying_prices
                .insert(tick.symbol(), tick.underlying_price());
            self.current_date = tick.date();

            if day_changed {
                // force close anything that is expiring and that the model
                // didn't already close the last trading day.  do this before
                // we reset the quotes so that the last trading day's quotes
                // are used when closing positions.
                self.close_expired_positions();

                let key_count = self.quotes.keys().len();

                if key_count > self.quote_map_capacity {
                    self.quote_map_capacity = key_count;
                }

                self.quotes = FnvHashMap::with_capacity_and_hasher(
                    self.quote_map_capacity,
                    Default::default(),
                );
            }

            // ----- next day --------------------------------------------------

            // TODO: maybe check that the ticks are in chronological order here?
            // TODO: record last_tick time on struct

            // update quote for this option
            self.quotes.insert(tick.name(), Quote::new(&tick));

            self.ticks_processed += 1;
        }

        self.close_all_positions();
    }

    fn close_expired_positions(&mut self) {
        if self.positions.is_empty() {
            return;
        }

        let mut orders = vec![];

        for (option_name, position) in &self.positions {
            if position.is_open() && position.is_expired(self.current_date) {
                println!("closing position {} due to expiration:", option_name);

                let quote = self.quote_for(position.name()).unwrap();
                let quantity = position.quantity();
                let action;
                let price;

                // TODO: call OrderFiller's logic here

                let order = if position.is_long() {
                    action = "sell";
                    price = quote.bid();

                    Order::new_sell_close_order(&quote, quantity, price)
                } else {
                    action = "buy";
                    price = quote.ask();

                    Order::new_buy_close_order(&quote, quantity, price)
                };

                let commish = self.commission_schedule.commission_for(&order);
                let mut filled_order = order;
                filled_order.filled_at(quote.midpoint_price(), commish, &quote, self.current_date);

                let total = filled_order.margin_requirement(price) + commish;

                println!(
                    "  {}ing contracts @ {} + {} commission ({} total)",
                    action,
                    filled_order.fill_price(),
                    commish,
                    total,
                );

                orders.push(filled_order);
            }
        }

        for order in orders {
            if !self.process_order(order) {
                panic!("failed to process_order... margin call?");
            }
        }
    }

    pub fn ticks_processed(&self) -> i64 {
        self.ticks_processed
    }

    pub fn current_date(&self) -> DateTime<Utc> {
        self.current_date
    }

    pub fn account_balance(&self) -> Money {
        self.balance
    }

    pub fn quote_for(&self, option_name: Rc<str>) -> Option<Quote> {
        match self.quotes.get(&option_name) {
            Some(q) => Some(q.clone()),
            None => None,
        }
    }

    // TODO: positions have a correct cost basis

    pub fn process_order(&mut self, order: Order) -> bool {
        // TODO: assign a unique id to each order

        // TODO: exit cleanly instead of exploding?
        let quote = self.quote_for(order.option_name()).unwrap();

        // println!("Order received: {}", order.summary());

        // TODO: ensure that days remaining is > 0
        //       since we only buy at end of day, if there are no days left
        //       the the contract is _already_ expired.

        // TODO: validate that the option_name in the order actually exists

        let commish = self.commission_schedule.commission_for(&order);

        // TODO: actually look at the required limit on the order
        let fill_price = quote.midpoint_price();
        let required_margin = order.margin_requirement(fill_price);

        if order.is_buy() {
            if required_margin + commish > self.balance {
                println!(
                    "not enough money (need {} + {} commission, have {})",
                    required_margin,
                    commish,
                    self.balance,
                );
                return false;
            }
        }

        // ----- fill the order ------------------------------------------------------

        let sign = if order.is_buy() { ">>" } else { "<<" };
        let call = if quote.is_call() { "CALL" } else { "PUT" };

        let mut filled_order = order;

        // fill the order and record it
        filled_order.filled_at(fill_price, commish, &quote, self.current_date);

        let cost_basis = filled_order.canonical_cost_basis();

        let key = filled_order.option_name();

        let filled_order_rc = Rc::from(filled_order);

        self.positions
            .entry(key)
            .or_insert(Position::new(&quote))
            .apply_order(filled_order_rc);

        let original_balance = self.balance;

        // TODO: put this stuff in an apply_order() function or something
        self.balance += cost_basis;

        // apply commission to balance and running total of paid commission
        self.balance -= commish;
        self.commission_paid += commish;

        println!("{} {} ORDER FILLED.", sign, call);
        println!(
            "Strike: {} - Commission: {} - Old balance: {} - New balance: {}",
            quote.strike_price(),
            commish,
            original_balance,
            self.balance,
        );
        println!(
            "   Underlying: {} - Bid: {} - Ask: {} - Expiration: {} days",
            self.underlying_price_for("AAPL"),
            quote.bid(),
            quote.ask(),
            quote.days_to_expiration(self.current_date),
        );

        true
    }

    // TODO: maybe don't sort this all the time?
    //       open_positions() consumes this after it's sorted, etc.
    pub fn positions(&self) -> Vec<Position> {
        let mut ps: Vec<Position> = self.positions.clone().into_iter().map(|(_, p)| p).collect();
        ps.sort_by(|a, b| a.name().cmp(&b.name()));
        ps
    }

    pub fn open_positions(&self) -> Vec<Position> {
        self.positions()
            .into_iter()
            .filter(|p| p.is_open())
            .collect()
    }

    // TODO: switch back to reference version once pmcc.rs isn't borrowing mutably and immutably
    // pub fn positions(&self) -> Vec<&Position> {
    //     let mut ps: Vec<&Position> = self.positions.iter().map(|(_, p)| p).collect();
    //     ps.sort_by(|a, b| a.name().cmp(&b.name()));
    //     ps
    // }

    // pub fn open_positions(&self) -> Vec<&Position> {
    //     self.positions()
    //         .into_iter()
    //         .filter(|p| p.is_open())
    //         .collect()
    // }

    pub fn total_order_count(&self) -> i32 {
        self.orders.len() as i32
    }

    pub fn commission_paid(&self) -> Money {
        self.commission_paid
    }

    pub fn close_all_positions(&mut self) {
        println!("TODO: close all open positions at last price");
        println!("");
    }
}
