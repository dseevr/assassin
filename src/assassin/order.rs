use std::rc::Rc;

use assassin::quote::Quote;

extern crate chrono;
use self::chrono::prelude::*;

extern crate greenback;
use greenback::Greenback as Money;

#[derive(Clone)]
pub struct Order {
    symbol: Rc<str>,
    name: Rc<str>,
    buy: bool,
    open: bool,
    quantity: i32,
    limit: Money,
    strike_price: Money,
    // date: DateTime<Utc>, // TODO: flesh this out

    // filled in by the broker when an order is filled
    quote: Option<Quote>,
    fill_price: Option<Money>,
    commission: Option<Money>,
    filled_date: Option<DateTime<Utc>>, // open date could have been in the past if GTC

    closed_by_broker: bool,
}

impl Order {
    pub fn is_buy(&self) -> bool {
        self.buy
    }

    #[allow(dead_code)]
    pub fn is_sell(&self) -> bool {
        !self.is_buy()
    }

    pub fn closed_by_broker(&self) -> bool {
        self.closed_by_broker
    }

    pub fn commission(&self) -> Money {
        match self.commission {
            Some(c) => c,
            None => panic!("can't get commission on unfilled order"),
        }
    }

    pub fn filled_at(&mut self, price: Money, quote: &Quote, date: DateTime<Utc>) {
        self.quote = Some(quote.clone());
        self.fill_price = Some(price);
        self.filled_date = Some(date);
    }

    pub fn set_commission(&mut self, commish: Money) {
        self.commission = Some(commish);
    }

    pub fn fill_price(&self) -> Money {
        match self.fill_price {
            Some(fp) => fp,
            None => panic!("can't get fill_price on unfilled order"),
        }
    }

    pub fn buy_or_sell_string(&self) -> &str {
        if self.buy {
            "BUY"
        } else {
            "SELL"
        }
    }

    // "AAPL: BUY 10 CALL $150 STRIKE at LIMIT $2.50"
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} {} {} {} STRIKE at LIMIT {}",
            self.symbol(),
            self.buy_or_sell_string(),
            self.quantity,
            self.strike_price,
            self.limit,
        )
    }

    pub fn option_name(&self) -> Rc<str> {
        Rc::clone(&self.name)
    }

    pub fn set_closed_by_broker(&mut self) {
        self.closed_by_broker = true;
    }

    // TODO: order expiration date would be the ORDER'S expiration date
    //       good til canceled, day only, etc.
    // pub fn expiration_date(&self) -> DateTime<Utc> {
    // 	self.quote.expiration_date()
    // }

    pub fn new_buy_open_order(quote: &Quote, quantity: i32, limit: Money) -> Order {
        if quantity <= 0 {
            panic!("quantity must be > 0 (got {})", quantity);
        }

        if limit < Money::zero() {
            panic!("limit must be >= 0.0 (got {})", limit);
        }

        Order {
            symbol: quote.symbol(),
            name: quote.name(),
            buy: true,
            open: true,
            quantity: quantity,
            limit: limit,
            strike_price: quote.strike_price(),

            // filled in later by broker if order is filled
            fill_price: None,
            commission: None,
            quote: None,
            filled_date: None,
            closed_by_broker: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_sell_open_order(quote: &Quote, quantity: i32, limit: Money) -> Order {
        let mut o = Order::new_buy_open_order(quote, quantity, limit);
        o.buy = false;

        o
    }

    pub fn new_buy_close_order(quote: &Quote, quantity: i32, limit: Money) -> Order {
        let mut o = Order::new_buy_open_order(quote, quantity, limit);
        o.open = false;

        o
    }

    pub fn new_sell_close_order(quote: &Quote, quantity: i32, limit: Money) -> Order {
        let mut o = Order::new_buy_open_order(quote, quantity, limit);
        o.buy = false;
        o.open = false;

        o
    }

    #[allow(dead_code)]
    pub fn buy_to_open(&self) -> bool {
        self.buy && self.open
    }

    #[allow(dead_code)]
    pub fn sell_to_open(&self) -> bool {
        !self.buy && self.open
    }

    pub fn buy_to_close(&self) -> bool {
        self.buy && !self.open
    }

    #[allow(dead_code)]
    pub fn sell_to_close(&self) -> bool {
        !self.buy && !self.open
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    #[allow(dead_code)]
    pub fn is_close(&self) -> bool {
        !self.is_open()
    }

    pub fn margin_requirement(&self, price: Money) -> Money {
        price * 100 * self.quantity
    }

    // TODO: double check that this is doing the right thing
    pub fn cost_basis(&self) -> Money {
        self.fill_price.unwrap() * 100 * self.quantity
    }

    #[allow(dead_code)]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    #[allow(dead_code)]
    pub fn limit(&self) -> Money {
        self.limit
    }

    pub fn canonical_quantity(&self) -> i32 {
        if self.buy {
            self.quantity
        } else {
            -self.quantity
        }
    }

    pub fn canonical_cost_basis(&self) -> Money {
        if self.buy {
            Money::zero() - self.cost_basis()
        } else {
            self.cost_basis()
        }
    }

    pub fn unrealized_value(&self, quote: &Quote) -> Money {
        let price = if self.is_buy() {
            quote.bid()
        } else {
            quote.ask()
        };

        price * self.quantity
    }
}
