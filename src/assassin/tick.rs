use assassin::quote::Quote;

extern crate chrono;

use self::chrono::prelude::*;

// Symbol ExpirationDate AskPrice AskSize BidPrice BidSize LastPrice PutCall StrikePrice Volume ImpliedVolatility Delta  Gamma  Vega,    Rho OpenInterest UnderlyingPrice DataDate
// AAPL   2013-01-04     10.55            10.35            10.55     call    540         14292  0.295             0.7809 2.4778 11.9371      8666         549.03          2013-01-02
// AAPL,2013-01-04,10.55,,10.35,,10.55,call,540,14292,0.295,0.7809,2.4778,11.9371,,8666,549.03,2013-01-02
#[allow(dead_code)]
#[derive(Clone)]
pub struct Tick {
	symbol: String,
	expiration_date: DateTime<Utc>,
	formatted_expiration_date: String,
	ask: f64,
	bid: f64,
	last_price: f64,
	call: bool,
	strike_price: f64,
	volume: i32,
	implied_volatility: f64,
	delta: f64,
	gamma: f64,
	vega: f64,
	open_interest: i32,
	underlying_price: f64,
	date: DateTime<Utc>,

	// TODO: bool or type for american vs european
}

impl Tick {

	pub fn new(
		symbol: String,
		expiration_date: DateTime<Utc>,
		ask: f64,
		bid: f64,
		last_price: f64,
		call: bool,
		strike_price: f64,
		volume: i32,
		implied_volatility: f64,
		delta: f64,
		gamma: f64,
		vega: f64,
		open_interest: i32,
		underlying_price: f64,
		date: DateTime<Utc>,
	) -> Tick {
		let formatted_expiration_date = {
			let year = expiration_date.year();
			let month = expiration_date.month();
			let day = expiration_date.day();

			format!("{}{}{}", year, month, day)
		};

		Tick{
			symbol: symbol,
			expiration_date: expiration_date,
			formatted_expiration_date: formatted_expiration_date,
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
		}
	}

	pub fn strike_price(&self) -> f64 {
		self.strike_price
	}

	pub fn quote(&self) -> Quote {
		Quote::new(&self)
	}

	// See: https://en.wikipedia.org/wiki/Option_naming_convention#Proposed_revision
	// e.g., CSCO171117C00019000
	pub fn name(&self) -> String {
		format!(
			"{symbol}{date}{t}{price:>0width$}0",
			symbol = self.symbol,
			date = self.formatted_expiration_date,
			t = if self.call { "C" } else { "P" },
			price = self.strike_price * 100.0,
			width = 7,
		)
	}

	pub fn days_until_expiration(&self) -> i32 {
		// TODO: use https://docs.rs/chrono/0.4.0/chrono/trait.Datelike.html#method.num_days_from_ce until a better solution is found
		self.expiration_date.num_days_from_ce() - self.date.num_days_from_ce()
	}

	pub fn midpoint_price(&self) -> f64 {
		(self.ask + self.bid) / 2.0
	}

	// TODO: move this stuff over to Quote
	pub fn intrinsic_value(&self) -> f64 {
		if self.call {
			if self.underlying_price > self.strike_price {
				self.underlying_price - self.strike_price
			} else {
				0.0
			}
		} else {
			if self.underlying_price < self.strike_price {
				self.strike_price - self.underlying_price
			} else {
				0.0
			}
		}
	}

	// TODO: move this stuff over to Quote
	pub fn extrinsic_value(&self) -> f64 {
		self.midpoint_price() - self.intrinsic_value()
	}

	// TODO: move this stuff over to Quote
	pub fn value_ratio(&self) -> f64 {
		// TODO: if i_value is 0, this is division by 0 and becomes infinity.
		//       see if we should return an Option<f64> in light of that...
		(self.extrinsic_value() / self.intrinsic_value()) * 100.0
	}

	// TODO: move this stuff over to Quote
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

	pub fn symbol(&self) -> &str {
		&self.symbol
	}

	pub fn bid(&self) -> f64 {
		self.bid
	}

	pub fn ask(&self) -> f64 {
		self.ask
	}
}