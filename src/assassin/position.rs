use std::rc::Rc;

use assassin::order::Order;
use assassin::quote::Quote;

extern crate chrono;
use self::chrono::prelude::*;

extern crate greenback;
use greenback::Greenback as Money;

#[derive(Clone)]
pub struct Position {
    name: Rc<str>,
    symbol: Rc<str>,
    quantity: i32,
    expiration_date: DateTime<Utc>,
    orders: Vec<Rc<Order>>,
}

impl Position {
    // NOTE: apply_order() still needs to be called afterwards.
    //       order is only used to set the name/symbol/expiration date
    pub fn new(quote: &Quote) -> Position {
        Position {
            name: quote.name(),
            symbol: quote.symbol(),
            quantity: 0,
            expiration_date: quote.expiration_date(),
            // don't set the order here because it gets applied in
            // apply_order() below.
            orders: vec![],
        }
    }

    pub fn broker_closed_order_count(&self) -> i32 {
        self.orders
            .iter()
            .filter(|o| o.closed_by_broker())
            .collect::<Vec<&Rc<Order>>>()
            .len() as i32
    }

    // OPTIMIZE: this can be updated when orders are applied
    pub fn realized_profit(&self) -> Money {
        self.orders.iter().fold(Money::zero(), |sum, o|
            // NOTE: a buy order really changes the position's value
            //       by a negative amount because it's tying up capital
            //       in a debit. a sell order grants a credit and is thus
            //       a positive value.
            //
            //       canonical_quantity() returns the correct values
            //       (i.e., a buy is 10, a sell is -10) for quantity, but
            //       we want to invert this because we want a buy to be
            //       a debit and a sell to be a credit.
            sum - (o.fill_price() * 100 * o.canonical_quantity()))
    }

    // OPTIMIZE: this can be updated when orders are applied
    pub fn commission_paid(&self) -> Money {
        self.orders.iter().map(|o| o.commission()).sum()
    }

    pub fn symbol(&self) -> Rc<str> {
        Rc::clone(&self.symbol)
    }

    pub fn name(&self) -> Rc<str> {
        Rc::clone(&self.name)
    }

    pub fn orders(&self) -> &Vec<Rc<Order>> {
        &self.orders
    }

    pub fn order_count(&self) -> i32 {
        self.orders.len() as i32
    }

    pub fn apply_order(&mut self, order: Rc<Order>) {
        self.quantity += order.canonical_quantity();
        self.orders.push(order);
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn expiration_date(&self) -> DateTime<Utc> {
        self.expiration_date
    }

    pub fn is_long(&self) -> bool {
        self.quantity > 0
    }

    #[allow(dead_code)]
    pub fn is_short(&self) -> bool {
        !self.is_long()
    }

    #[allow(dead_code)]
    pub fn is_flat(&self) -> bool {
        self.quantity == 0
    }

    pub fn is_open(&self) -> bool {
        let mut open = 0;
        let mut closed = 0;

        for o in self.orders.iter() {
            if o.is_open() {
                open += 1;
            } else {
                closed += 1;
            }
        }

        open != closed
    }

    #[allow(dead_code)]
    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }

    // TODO: add expires_on() and use in Broker.process_order()

    pub fn is_expired(&self, current_date: DateTime<Utc>) -> bool {
        // < instead of <= because we update the current date in the broker
        // _before_ calling this function.  if we used <=, it would close
        // positions which expire on the current trading day before the
        // model's logic has run.
        self.expiration_date.num_days_from_ce() < current_date.num_days_from_ce()
    }

    #[allow(dead_code)]
    pub fn current_value(&self) -> Money {
        self.orders.iter().map(|o| o.canonical_cost_basis()).sum()
    }
}
