use std::rc::Rc;

use assassin::order::Order;
use assassin::quote::Quote;

extern crate chrono;
use self::chrono::prelude::*;

extern crate greenback;
use greenback::Greenback as Money;

#[derive(Clone)]
pub struct FilledOrder {
    order: Order,

    fill_quote: Quote,
    fill_price: Money,
    fill_date: DateTime<Utc>,
    commission: Money,

    closed_by_broker: bool,
}

impl FilledOrder {
    // consumes an Order, Quote, fill price (Money), and fill date and produces a FilledOrder
    pub fn new(
        order: Order,
        quote: &Quote,
        fill_price: Money,
        fill_date: DateTime<Utc>,
    ) -> FilledOrder {
        let filled_order = FilledOrder {
            order: order,
            fill_quote: quote.clone(),
            fill_price: fill_price,
            fill_date: fill_date,
            commission: Money::zero(), // TODO: pass in commission and set in this function
            closed_by_broker: false,
        };

        filled_order
    }

    pub fn set_commission(&mut self, commish: Money) {
        if commish < Money::zero() {
            panic!("commission can't be negative (got: {})", commish);
        }

        self.commission = commish;
    }

    pub fn commission(&self) -> Money {
        self.commission
    }

    pub fn fill_price(&self) -> Money {
        self.fill_price
    }

    pub fn closed_by_broker(&self) -> bool {
        self.closed_by_broker
    }

    pub fn set_closed_by_broker(&mut self) {
        self.closed_by_broker = true;
    }

    // TODO: double check that this is doing the right thing
    pub fn cost_basis(&self) -> Money {
        // TODO: make 100 a constant somewhere
        self.fill_price * 100 * self.order.quantity()
    }

    pub fn canonical_cost_basis(&self) -> Money {
        if self.is_buy() {
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

        price * 100 * self.canonical_quantity()
    }

    // ===== proxied functions ==========================================================

    pub fn buy_or_sell_string(&self) -> &str {
        self.order.buy_or_sell_string()
    }

    pub fn quantity(&self) -> i32 {
        self.order.quantity()
    }

    pub fn option_name(&self) -> Rc<str> {
        self.order.option_name()
    }

    pub fn margin_requirement(&self, price: Money) -> Money {
        self.order.margin_requirement(price)
    }

    pub fn canonical_quantity(&self) -> i32 {
        self.order.canonical_quantity()
    }

    pub fn is_buy(&self) -> bool {
        self.order.is_buy()
    }

    pub fn is_sell(&self) -> bool {
        self.order.is_sell()
    }

    pub fn buy_to_open(&self) -> bool {
        self.order.buy_to_open()
    }

    pub fn sell_to_open(&self) -> bool {
        self.order.sell_to_open()
    }

    pub fn buy_to_close(&self) -> bool {
        self.order.buy_to_close()
    }

    pub fn sell_to_close(&self) -> bool {
        self.order.sell_to_close()
    }

    pub fn is_open(&self) -> bool {
        self.order.is_open()
    }

    pub fn is_close(&self) -> bool {
        self.order.is_close()
    }
}
