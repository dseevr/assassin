#[derive(Clone)]
pub struct Order {
	symbol: String,
	buy: bool,
	open: bool,
	strike: f64,
	quantity: i32,
	limit: f64,
}

impl Order {
	pub fn new_buy_open_order(symbol: String, strike: f64, quantity: i32, limit: f64) -> Order {
		if symbol.len() == 0 {
			panic!("symbol must be > 0 characters");
		}

		if strike <= 0.0 {
			panic!("strike must be > 0.0 (got {})", strike);
		}

		if quantity <= 0 {
			panic!("quantity must be > 0 (got {})", quantity);
		}

		if limit <= 0.0 {
			panic!("limit must be > 0.0 (got {})", limit);
		}

		Order{
			symbol: symbol,
			buy: true,
			open: true,
			strike: strike,
			quantity: quantity,
			limit: limit,
		}
	}

	pub fn new_sell_open_order(symbol: String, strike: f64, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(symbol, strike, quantity, limit);
		o.buy = false;

		o
	}

	pub fn new_buy_close_order(symbol: String, strike: f64, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(symbol, strike, quantity, limit);
		o.open = false;

		o
	}

	pub fn new_sell_close_order(symbol: String, strike: f64, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(symbol, strike, quantity, limit);
		o.buy = false;
		o.open = false;

		o
	}

	pub fn buy_to_open(&self) -> bool {
		self.buy && self.open
	}

	pub fn sell_to_open(&self) -> bool {
		! self.buy && self.open
	}

	pub fn buy_to_close(&self) -> bool {
		self.buy && ! self.open
	}

	pub fn sell_to_close(&self) -> bool {
		! self.buy && ! self.open
	}

	pub fn cost_basis(&self) -> f64 {
		self.quantity as f64 * self.limit * 100.0 // TODO: assumes 100...
	}

	pub fn symbol(&self) -> String {
		self.symbol.clone()
	}

	pub fn quantity(&self) -> i32 {
		self.quantity
	}

	pub fn limit(&self) -> f64 {
		self.limit
	}

	pub fn canonical_quantity(&self) -> i32 {
		if self.buy {
			self.quantity
		} else {
			- self.quantity
		}
	}

	pub fn canonical_cost_basis(&self) -> f64 {
		if self.buy {
			// debit
			- self.cost_basis()
		} else {
			// credit
			self.cost_basis()
		}
	}
}
