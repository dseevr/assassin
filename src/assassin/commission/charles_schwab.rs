
use assassin::traits::*;
use assassin::order::Order;

pub struct CharlesSchwab {
	base_fee: f32,
	per_contract: f32
}

impl CharlesSchwab {
	pub fn new() -> CharlesSchwab {
		CharlesSchwab{
			base_fee: 4.95,
			per_contract: 0.65
		}
	}
}

// https://www.schwab.com/public/schwab/active_trader/pricing
impl Commission for CharlesSchwab {
	fn commission_for(&self, order: &Order) -> f32 {
		if order.buy_to_close() && order.fill_price() <= 0.05 {
			0.0 // no commission on buy-to-close for <= $0.05
		} else {
			self.base_fee + (order.quantity() as f32 * self.per_contract)
		}
	}
}