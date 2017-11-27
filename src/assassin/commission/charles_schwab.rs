use assassin::order::Order;
use assassin::traits::*;

extern crate greenback;
use greenback::Greenback as Money;

pub struct CharlesSchwab {
    base_fee: Money,
    per_contract: Money
}

impl CharlesSchwab {
    pub fn new() -> CharlesSchwab {
        CharlesSchwab{
            base_fee: Money::new(4, 95),
            per_contract: Money::new(0, 65)
        }
    }
}

// https://www.schwab.com/public/schwab/active_trader/pricing
impl Commission for CharlesSchwab {
    fn commission_for(&self, order: &Order) -> Money {
        if order.buy_to_close() && order.fill_price() <= Money::new(0, 5) {
            Money::zero() // no commission on buy-to-close for <= $0.05
        } else {
            self.base_fee + self.per_contract * order.quantity()
        }
    }
}
