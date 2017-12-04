use std::rc::Rc;

use assassin::quote::Quote;

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
    // date: DateTime<Utc>, // TODO: flesh this out (Date order placed... could be GTC, etc.)
}

impl Order {
    pub fn is_buy(&self) -> bool {
        self.buy
    }

    pub fn is_sell(&self) -> bool {
        !self.is_buy()
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

    pub fn buy_to_open(&self) -> bool {
        self.buy && self.open
    }

    pub fn sell_to_open(&self) -> bool {
        !self.buy && self.open
    }

    pub fn buy_to_close(&self) -> bool {
        self.buy && !self.open
    }

    pub fn sell_to_close(&self) -> bool {
        !self.buy && !self.open
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn is_close(&self) -> bool {
        !self.is_open()
    }

    pub fn margin_requirement(&self, price: Money) -> Money {
        price * 100 * self.quantity
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
}

#[cfg(test)]
mod tests {

    use super::*;

    fn dummy_quote(bid: Money, ask: Money) -> Quote {
        Quote::new(
            "symbol".to_string(), // symbol
            Utc::now(),           // expiration date
            ask,                  // ask
            bid,                  // bid
            Money::new(1, 0),     // last_price
            true,                 // call
            Money::new(100, 0),   // strike_price
            0,                    // volume
            0.0,                  // IV
            0.0,                  // delta
            0.0,                  // gamma
            0.0,                  // vega
            0,                    // open interest
            Money::new(101, 0),   // underlying
            Utc::now(),           // date (of quote)
        )
    }

    fn filled_order(quote: &Quote) -> Order {
        Order {
            symbol: quote.symbol(),
            name: quote.name(),
            buy: true,
            open: true,
            quantity: 10,
            limit: Money::new(1, 0),
            strike_price: Money::new(1, 0),
            quote: Some(quote.clone()),
            fill_price: Some(quote.ask()),
            commission: Some(Money::zero()),
            filled_date: Some(Utc::now()),
            closed_by_broker: false,
        }
    }

    #[test]
    fn test_unrealized_value() {
        let m1 = Money::new(1, 1);
        let m2 = Money::new(1, 2);

        let q1 = dummy_quote(m1, m2); // bought at 1.02
        let q2 = dummy_quote(Money::new(1, 3), Money::new(1, 4)); // selling at 1.02

        let o = filled_order(&q1);

        let cost_basis = o.cost_basis();
        let unrealized = o.unrealized_value(&q1);
        let profit = o.unrealized_value(&q2) - o.cost_basis();

        println!(
            "cost_basis: {} unrealized: {} profit: {}",
            cost_basis,
            unrealized,
            profit
        );

        let d = m2 - m1;
        let difference = d * 100 * 10; // 10 contracts (from dummy_quote())

        println!("d: {} difference: {}", d, difference);

        println!("{} == {} ?", cost_basis - difference, unrealized);
        assert!(cost_basis - difference == unrealized); // selling immediately is a $0.01/share loss

        println!("profit: {} == {} ?", profit, difference);
        assert!(profit == difference); // selling at q2 is a $0.01/share profit
    }
}
