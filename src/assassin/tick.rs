use std::rc::Rc;

extern crate chrono;
use self::chrono::prelude::*;

extern crate greenback;
use greenback::Greenback as Money;

// Symbol ExpirationDate AskPrice AskSize BidPrice BidSize LastPrice PutCall StrikePrice Volume
// AAPL   2013-01-04     10.55            10.35            10.55     call    540         14292
// AAPL,2013-01-04,10.55,,10.35,,10.55,call,540,14292,0.295,0.7809,2.4778,11.9371,,8666,549.03,

// ImpliedVolatility Delta  Gamma  Vega,    Rho OpenInterest UnderlyingPrice DataDate
// 0.295             0.7809 2.4778 11.9371      8666         549.03          2013-01-02
// 2013-01-02
#[allow(dead_code)]
#[derive(Clone)]
pub struct Tick {
    symbol: Rc<str>,
    expiration_date: DateTime<Utc>,
    ask: Money,
    bid: Money,
    last_price: Money,
    call: bool,
    strike_price: Money,
    volume: i32,
    implied_volatility: f32,
    delta: f32,
    gamma: f32,
    vega: f32,
    open_interest: i32,
    underlying_price: Money,
    date: DateTime<Utc>,
    name: Rc<str>,

    // TODO: bool or type for american vs european
}

impl Tick {
    pub fn new(
        symbol: String,
        expiration_date: DateTime<Utc>,
        ask: Money,
        bid: Money,
        last_price: Money,
        call: bool,
        strike_price: Money,
        volume: i32,
        implied_volatility: f32,
        delta: f32,
        gamma: f32,
        vega: f32,
        open_interest: i32,
        underlying_price: Money,
        date: DateTime<Utc>,
    ) -> Tick {
        let name = format!(
            "{symbol}{year}{month}{day}{t}{price:>0width$}0",
            symbol = symbol,
            year = expiration_date.year(),
            month = expiration_date.month(),
            day = expiration_date.day(),
            t = if call { "C" } else { "P" },
            // this used to be multiplied by 100 but raw_value() is the same thing
            price = strike_price.raw_value(),
            width = 7,
        );

        let symbol_ref: &str = &symbol;
        let name_ref: &str = &name;

        Tick {
            symbol: Rc::from(symbol_ref),
            expiration_date: expiration_date,
            ask: ask,
            bid: bid,
            last_price: last_price,
            call: call,
            strike_price: strike_price,
            volume: volume,
            implied_volatility: implied_volatility,
            delta: delta,
            gamma: gamma,
            vega: vega,
            open_interest: open_interest,
            underlying_price: underlying_price,
            date: date,
            name: Rc::from(name_ref),
        }
    }

    pub fn underlying_price(&self) -> Money {
        self.underlying_price
    }

    pub fn is_call(&self) -> bool {
        self.call
    }

    #[allow(dead_code)]
    pub fn is_put(&self) -> bool {
        !self.is_call()
    }

    pub fn strike_price(&self) -> Money {
        self.strike_price
    }

    // See: https://en.wikipedia.org/wiki/Option_naming_convention#Proposed_revision
    // e.g., CSCO171117C00019000
    pub fn name(&self) -> Rc<str> {
        Rc::clone(&self.name)
    }

    #[allow(dead_code)]
    pub fn days_until_expiration(&self) -> i32 {
        self.expiration_date.num_days_from_ce() - self.date.num_days_from_ce()
    }

    #[allow(dead_code)]
    pub fn midpoint_price(&self) -> Money {
        (self.ask + self.bid) / 2
    }

    #[allow(dead_code)]
    pub fn intrinsic_value(&self) -> Money {
        if self.call {
            if self.underlying_price > self.strike_price {
                self.underlying_price - self.strike_price
            } else {
                Money::zero()
            }
        } else {
            if self.underlying_price < self.strike_price {
                self.strike_price - self.underlying_price
            } else {
                Money::zero()
            }
        }
    }

    #[allow(dead_code)]
    pub fn extrinsic_value(&self) -> Money {
        self.midpoint_price() - self.intrinsic_value()
    }

    #[allow(dead_code)]
    pub fn value_ratio(&self) -> f32 {
        // TODO: if i_value is 0, this is division by 0 and becomes infinity.
        //       see if we should return an Option<Money> in light of that...

        let extrinsic = self.extrinsic_value().raw_value() as f32;
        let intrinsic = self.intrinsic_value().raw_value() as f32;

        (extrinsic / intrinsic) / 100.0
    }

    #[allow(dead_code)]
    pub fn print_deets(&self) {
        println!("=======================");
        println!("name: {}", self.name());
        println!("spread: {}", self.ask - self.bid);
        println!("intrinsic: {}", self.intrinsic_value());
        println!("extrinsic: {}", self.extrinsic_value());
        println!("value ratio: {:.2}%", self.value_ratio());
        println!("last price: {}", self.last_price);
        println!("underlying price: {}", self.underlying_price);
        println!("date: {} expiration: {}", self.date, self.expiration_date);
        println!("days left: {}", self.days_until_expiration());
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn expiration_date(&self) -> DateTime<Utc> {
        self.expiration_date
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
}
