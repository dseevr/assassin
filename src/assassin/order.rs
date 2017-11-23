use assassin::quote::Quote;
use assassin::util::*;

#[derive(Clone)]
pub struct Order {
	symbol: String,
	name: String,
	buy: bool,
	open: bool,
	quantity: i32,
	limit: f64,
	strike_price: f64,
	// date: DateTime<Utc>, // TODO: flesh this out

	// filled in by the broker when an order is accepted
	quote: Option<Quote>,
	fill_price: Option<f64>,
	commission: Option<f64>,
}

impl Order {
	pub fn is_buy(&self) -> bool {
		self.buy
	}

	pub fn is_sell(&self) -> bool {
		! self.is_buy()
	}

	pub fn commission(&self) -> f64 {
		match self.commission {
			Some(c) => c,
			None => panic!("can't get commission on unfilled order")
		}
	}

	pub fn filled_at(&mut self, price: f64, commish: f64, quote: &Quote) {
		self.quote = Some(quote.clone());
		self.fill_price = Some(price);
		self.commission = Some(commish);
	}

	pub fn fill_price(&self) -> f64 {
		match self.fill_price {
			Some(fp) => fp,
			None => panic!("can't get fill_price on unfilled order")
		}
	}

	pub fn buy_or_sell_string(&self) -> &str {
		if self.buy { "BUY" } else { "SELL" }
	}

	// "AAPL: BUY 10 CALL $150 STRIKE at LIMIT $2.50"
	pub fn summary(&self) -> String {
		format!(
			"{} {} {} {} STRIKE at LIMIT {}",
			self.symbol(),
			self.buy_or_sell_string(),
			self.quantity,
			format_money(self.strike_price),
			format_money(self.limit),
		)
	}

	pub fn option_name(&self) -> &str {
		&self.name
	}

	// TODO: order expiration date would be the ORDER'S expiration date
	//       good til canceled, day only, etc.
	// pub fn expiration_date(&self) -> DateTime<Utc> {
	// 	self.quote.expiration_date()
	// }

	pub fn new_buy_open_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		if quantity <= 0 {
			panic!("quantity must be > 0 (got {})", quantity);
		}

		if limit < 0.0 {
			panic!("limit must be >= 0.0 (got {})", limit);
		}

		Order{
			symbol: quote.symbol().to_string(),
			name: quote.name().to_string(),
			buy: true,
			open: true,
			quantity: quantity,
			limit: limit,
			strike_price: quote.strike_price(),

			// filled in later by broker if order is filled
			fill_price: None,
			commission: None,
			quote: None,
		}
	}

	pub fn new_sell_open_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(quote, quantity, limit);
		o.buy = false;

		o
	}

	pub fn new_buy_close_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(quote, quantity, limit);
		o.open = false;

		o
	}

	pub fn new_sell_close_order(quote: &Quote, quantity: i32, limit: f64) -> Order {
		let mut o = Order::new_buy_open_order(quote, quantity, limit);
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

	pub fn margin_requirement(&self, price: f64) -> f64 {
		self.quantity as f64 * price * 100.0
	}

	pub fn cost_basis(&self) -> f64 {
		self.quantity as f64 * self.fill_price.unwrap() * 100.0
	}

	pub fn symbol(&self) -> &str {
		&self.symbol
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
