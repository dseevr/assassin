use assassin::order::Order;

#[derive(Clone)]
pub struct Position {
	symbol: String,
	quantity: i32,
}

impl Position {
	pub fn new(order: &Order) -> Position {
		Position{
			symbol: order.symbol(),
			quantity: 0,
		}
	}

	pub fn apply_order(&mut self, order: &Order) {
		self.quantity += order.canonical_quantity()
	}

	pub fn quantity(&self) -> i32 {
		self.quantity
	}
}
