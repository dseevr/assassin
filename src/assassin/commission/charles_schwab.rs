use assassin::money::Money;
use assassin::traits::*;
use assassin::order::Order;

pub struct CharlesSchwab {
	base_fee: Money,
	per_contract: Money
}

impl CharlesSchwab {
	pub fn new() -> CharlesSchwab {
		CharlesSchwab{
			base_fee: Money::new(4_95),
			per_contract: Money::new(65)
		}
	}
}

// https://www.schwab.com/public/schwab/active_trader/pricing
impl Commission for CharlesSchwab {
	fn commission_for(&self, order: &Order) -> Money {
		if order.buy_to_close() && order.fill_price() <= Money::new(5) {
			Money::zero() // no commission on buy-to-close for <= $0.05
		} else {
			self.base_fee + self.per_contract * order.quantity()
		}
	}
}