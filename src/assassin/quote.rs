#[derive(Clone)]
pub struct Quote {
	bid: f64,
	ask: f64,
	expiration_date: String,
	// TODO: depth, etc. if available
}

impl Quote {
	pub fn new(bid: f64, ask: f64, expiration_date: String) -> Quote {
		if bid > ask {
			panic!("got bid {} > ask {}", bid, ask);
		}

		Quote{
			bid: bid,
			ask: ask,
			expiration_date: expiration_date,
		}
	}

	pub fn bid(&self) -> f64 {
		self.bid
	}

	pub fn ask(&self) -> f64 {
		self.ask
	}

	pub fn expiration_date(&self) -> String {
		self.expiration_date.clone()
	}
}