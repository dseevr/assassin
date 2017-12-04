use std::rc::Rc;

use assassin::filled_order::FilledOrder;
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
    orders: Vec<FilledOrder>, // TODO: rename to filled_orders and store received Orders separately
    commission_schedule: Box<Commission>,
    commission_paid: Money,
    data_feed: Box<DataFeed>,
    // TODO: convert this into a FnvHashMap<Rc<str>, FnvHashMap<Rc<str>, Quote>>
    quotes: FnvHashMap<Rc<str>, Quote>,
    current_date: DateTime<Utc>,
    quotes_processed: i32,
    quote_map_capacity: usize,
    underlying_prices: FnvHashMap<Rc<str>, Money>,

    // statistics for simulation
    highest_realized_account_balance: Money,
    lowest_realized_account_balance: Money,
    highest_unrealized_account_balance: Money,
    lowest_unrealized_account_balance: Money,
    final_unrealized_account_balance: Money,

    // TODO: add vars for realized and unrealized high/low balances
    carried_over_quote: Option<Quote>,
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
            quotes_processed: 0,
            quote_map_capacity: 0,
            underlying_prices: FnvHashMap::default(),
            highest_realized_account_balance: initial_balance,
            lowest_realized_account_balance: initial_balance,
            highest_unrealized_account_balance: initial_balance,
            lowest_unrealized_account_balance: initial_balance,
            final_unrealized_account_balance: initial_balance,
            carried_over_quote: None,
        }
    }

    pub fn highest_realized_account_balance(&self) -> Money {
        self.highest_realized_account_balance
    }

    pub fn lowest_realized_account_balance(&self) -> Money {
        self.lowest_realized_account_balance
    }

    pub fn highest_unrealized_account_balance(&self) -> Money {
        self.highest_unrealized_account_balance
    }

    pub fn lowest_unrealized_account_balance(&self) -> Money {
        self.lowest_unrealized_account_balance
    }

    pub fn update_statistics(&mut self) {
        let current_value = self.balance;

        if current_value > self.highest_realized_account_balance {
            self.highest_realized_account_balance = current_value;
        } else if current_value < self.lowest_realized_account_balance {
            self.lowest_realized_account_balance = current_value;
        }

        let current_unrealized_value = self.unrealized_account_balance();

        // println!("current unrealized: {}", current_unrealized_value);

        if current_unrealized_value > self.highest_unrealized_account_balance {
            self.highest_unrealized_account_balance = current_unrealized_value;
        } else if current_unrealized_value < self.lowest_unrealized_account_balance {
            self.lowest_unrealized_account_balance = current_unrealized_value;
        }
    }

    pub fn underlying_price_for(&self, symbol: &str) -> Money {
        *self.underlying_prices.get(symbol).unwrap()
    }

    // TODO: this should only return quotes for the desired symbol
    pub fn quotes_for(&self, _symbol: &str) -> Vec<&Quote> {
        self.quotes.iter().map(|(_, q)| q).collect()
    }

    pub fn nearest_quotes_expiring_between_n_days(&self, min: i32, max: i32) -> Vec<&Quote> {
        if min < 0 {
            panic!("min must be >= 0 (got: {})", min);
        }

        if min > max {
            panic!("min must be > max (got: min {} and max {})", min, max);
        }

        let date = self.current_date;

        let mut expiring_quotes: Vec<&Quote> = self.quotes
            .iter()
            .map(|(_, q)| q)
            .into_iter()
            .filter(|q| {
                // TODO: should these be >= and <= ?
                q.days_to_expiration(date) > min && q.days_to_expiration(date) < max
            })
            .collect();

        expiring_quotes.sort_by_key(|a| {
            (a.days_to_expiration(date), a.is_call(), a.strike_price())
        });

        expiring_quotes
    }

    // pub fn calls_expiring_after_n_days(&self, days: i32) -> Vec<&Quote> {
    //     self.nearest_quotes_expiring_after_n_days(days)
    //         .into_iter()
    //         .filter(|q| q.is_call())
    //         .collect()
    // }

    // pub fn puts_expiring_after_n_days(&self, days: i32) -> Vec<&Quote> {
    //     self.nearest_quotes_expiring_after_n_days(days)
    //         .into_iter()
    //         .filter(|q| q.is_put())
    //         .collect()
    // }

    #[allow(dead_code)]
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

    pub fn process_simulation_data(&mut self) -> bool {
        println!("===== new day ==========================================");

        self.update_statistics();

        self.quotes =
            FnvHashMap::with_capacity_and_hasher(self.quote_map_capacity, Default::default());

        // if we aborted the main loop when a quote for the next day came in, apply it now
        if self.carried_over_quote.is_some() {
            let quote = self.carried_over_quote.clone().unwrap();

            self.underlying_prices
                .insert(quote.symbol(), quote.underlying_price());
            self.current_date = quote.date();

            self.quotes.insert(quote.name(), quote.clone());
            self.quotes_processed += 1;

            self.carried_over_quote = None;

            println!("inserted carried over data");
        }

        // manually consume the first quote here so we don't have to check
        // to see if it's the first quote every single time
        {
            let first_quote = self.data_feed.next_quote().unwrap();
            self.current_date = first_quote.date();
            self.quotes.insert(first_quote.name(), first_quote);
        }

        while let Some(quote) = self.data_feed.next_quote() {
            let day_changed = quote.date() != self.current_date;

            if day_changed {
                println!("day changed from {} to {}", quote.date(), self.current_date);
                // force close anything that is expiring and that the model
                // didn't already close the last trading day.  do this before
                // we reset the quotes so that the last trading day's quotes
                // are used when closing positions.
                self.close_expired_positions(quote.date());

                self.update_statistics();

                let key_count = self.quotes.keys().len();
                if key_count > self.quote_map_capacity {
                    self.quote_map_capacity = key_count;
                }

                self.carried_over_quote = Some(quote);

                return true;
            }

            // ----- next day --------------------------------------------------

            // TODO: maybe check that the quotes are in chronological order here?
            // TODO: record last_quote time on struct

            self.underlying_prices
                .insert(quote.symbol(), quote.underlying_price());
            self.current_date = quote.date();

            self.quotes.insert(quote.name(), quote);
            self.quotes_processed += 1;
        }

        self.final_unrealized_account_balance = self.unrealized_account_balance();

        self.close_all_open_positions();

        false
    }

    fn fill_order(&mut self, order: Order, quote: &Quote) {
        let action = if order.is_buy() { "buy" } else { "sell" };
        let sign = if order.is_buy() { ">>" } else { "<<" };
        let call = if quote.is_call() { "CALL" } else { "PUT" };

        // TODO: ensure that days remaining is > 0
        //       since we only buy at end of day, if there are no days left
        //       the the contract is _already_ expired.

        // TODO: call OrderFiller's logic here and use the limit on the Order
        // let fill_price = whatever;
        // let required_margin = filled_order.margin_requirement(fill_price);

        let fill_price = quote.midpoint_price();

        let mut filled_order = FilledOrder::new(order, &quote, fill_price, self.current_date);

        let commish = self.commission_schedule.commission_for(&filled_order);

        filled_order.set_commission(commish);
        filled_order.set_closed_by_broker();

        let total = filled_order.cost_basis() + filled_order.commission();
        let original_balance = self.unrealized_account_balance();

        // ===== validate that the account has enough money =============================

        // NOTE: don't reuse this below because we use canonical_cost_basis() when altering balances
        let required_margin = filled_order.cost_basis() + filled_order.commission();

        if filled_order.is_buy() && required_margin > self.balance {
            println!(
                "not enough money (need {}, have {})",
                required_margin,
                self.balance,
            );
        }

        // TODO: update values for balances and stuff

        let cost_basis = filled_order.canonical_cost_basis();
        let key = filled_order.option_name();
        let quantity = filled_order.quantity();
        let commish = filled_order.commission();
        let fill_price = filled_order.fill_price();

        let filled_order_rc = Rc::from(filled_order);

        // stick the FilledOrder onto the Position
        self.positions
            .entry(key)
            .or_insert(Position::new(&quote))
            .apply_order(filled_order_rc);

        self.balance += cost_basis;
        self.balance -= commish;
        self.commission_paid += commish;

        // ===== print details ==========================================================

        println!(
            "  {}ing contracts @ {} + {} commission ({} total)",
            action,
            fill_price,
            commish,
            total,
        );

        println!("{} {} ORDER FILLED.", sign, call);
        println!(
            "Strike: {} - Commission: {} - Old (un)balance: {} - New (un)balance: {}",
            quote.strike_price(),
            commish,
            original_balance,
            self.unrealized_account_balance(),
        );
        println!(
            "   Underlying: {} - Bid: {} - Ask: {} - Expiration: {} days - {} contracts",
            self.underlying_price_for("AAPL"),
            quote.bid(),
            quote.ask(),
            quote.days_to_expiration(self.current_date),
            quantity,
        );
    }

    pub fn quotes_processed(&self) -> i32 {
        self.quotes_processed
    }

    pub fn current_date(&self) -> DateTime<Utc> {
        self.current_date
    }

    pub fn account_balance(&self) -> Money {
        self.balance
    }

    pub fn unrealized_account_balance(&self) -> Money {
        self.balance
            + self.open_positions()
                .iter()
                .map(|p| p.current_value(&self.quote_for(p.name()).unwrap()))
                .sum()
    }

    pub fn quote_for(&self, option_name: Rc<str>) -> Option<Quote> {
        match self.quotes.get(&option_name) {
            Some(q) => Some(q.clone()),
            None => None,
        }
    }

    pub fn process_order(&mut self, order: Order) {
        // TODO: assign a unique id to each order

        let quote = self.quote_for(order.option_name()).unwrap();

        self.fill_order(order, &quote);
    }

    pub fn open_positions(&self) -> Vec<&Position> {
        let mut ps: Vec<&Position> = self.positions
            .iter()
            .map(|(_, p)| p)
            .filter(|p| p.is_open())
            .collect();
        ps.sort_by(|a, b| a.name().cmp(&b.name()));
        ps
    }

    pub fn positions(&self) -> Vec<&Position> {
        let mut ps: Vec<&Position> = self.positions.iter().map(|(_, p)| p).collect();
        ps.sort_by(|a, b| a.name().cmp(&b.name()));
        ps
    }

    pub fn total_order_count(&self) -> i32 {
        self.orders.len() as i32
    }

    pub fn commission_paid(&self) -> Money {
        self.commission_paid
    }

    // TODO: address mega duplication between close_expired_positions() and
    //       close_all_option_positions() without triggering mutable self crap

    fn close_expired_positions(&mut self, date: DateTime<Utc>) {
        let mut orders = vec![];

        // {
        //     println!("current_date: {}", self.current_date);
        //     for p in self.positions() {
        //         println!("position name: {}", p.name());
        //     }

        //     let quotes = self.quotes.iter().map(|(_, q)| q).collect::<Vec<&Quote>>();

        //     for q in quotes {
        //         println!("quote name: {}", q.name());
        //     }
        // }

        for position in self.open_positions() {
            if position.is_expired(date) {
                println!("before unwrap");
                let quote = self.quote_for(position.name()).unwrap();
                println!("after unwrap");
                let quantity = position.quantity().abs();

                // close at the worst possible price
                let order = if position.is_long() {
                    Order::new_sell_close_order(&quote, quantity, quote.bid())
                } else {
                    Order::new_buy_close_order(&quote, quantity, quote.ask())
                };

                orders.push((order, quote));
            }
        }

        for (o, q) in orders {
            self.fill_order(o, &q);
        }
    }

    fn close_all_open_positions(&mut self) {
        let mut orders = vec![];

        for position in self.open_positions() {
            let quote = self.quote_for(position.name()).unwrap();
            let quantity = position.quantity().abs();

            // close at the worst possible price
            let order = if position.is_long() {
                Order::new_sell_close_order(&quote, quantity, quote.bid())
            } else {
                Order::new_buy_close_order(&quote, quantity, quote.ask())
            };

            orders.push((order, quote));
        }

        for (o, q) in orders {
            self.fill_order(o, &q);
        }
    }
}
