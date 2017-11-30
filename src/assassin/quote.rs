use std::rc::Rc;

use assassin::tick::Tick;

extern crate chrono;
use self::chrono::prelude::*;

extern crate greenback;
use greenback::Greenback as Money;

#[derive(Clone)]
pub struct Quote {
    name: Rc<str>,
    symbol: Rc<str>,
    bid: Money,
    ask: Money,
    strike_price: Money,
    expiration_date: DateTime<Utc>,
    call: bool,
    // TODO: depth, etc. if available
}

impl Quote {
    pub fn new(tick: &Tick) -> Quote {
        if tick.bid() > tick.ask() {
            panic!("got bid {} > ask {}", tick.bid(), tick.ask());
        }

        Quote {
            name: tick.name(),
            symbol: tick.symbol(),
            bid: tick.bid(),
            ask: tick.ask(),
            strike_price: tick.strike_price(),
            expiration_date: tick.expiration_date(),
            call: tick.is_call(),
        }
    }

    pub fn is_call(&self) -> bool {
        self.call
    }

    #[allow(dead_code)]
    pub fn is_put(&self) -> bool {
        !self.is_call()
    }

    pub fn midpoint_price(&self) -> Money {
        (self.ask + self.bid) / 2
    }

    pub fn strike_price(&self) -> Money {
        self.strike_price
    }

    pub fn name(&self) -> Rc<str> {
        Rc::clone(&self.name)
    }

    pub fn symbol(&self) -> Rc<str> {
        Rc::clone(&self.symbol)
    }

    pub fn bid(&self) -> Money {
        self.bid
    }

    pub fn ask(&self) -> Money {
        self.ask
    }

    pub fn expiration_date(&self) -> DateTime<Utc> {
        self.expiration_date
    }
}
