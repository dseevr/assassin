use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::fs::File;

use assassin::tick::Tick;
use assassin::traits::*;

extern crate chrono;

use self::chrono::prelude::*;

pub struct DiscountOptionData {
	enumerator: Lines<BufReader<File>>,
}

impl DiscountOptionData {
	pub fn new(filename: &'static str) -> DiscountOptionData {
		let file = BufReader::new(File::open(filename).unwrap());
		let enumerator = file.lines();

		// eat the header row
		// enumerator.next();

		DiscountOptionData{
			enumerator: enumerator,
		}
	}
}

static TIME_TIMEZONE: &'static str = " 00:00:00 +00:00";
static CHRONO_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %z";

//    0         1            2      3        4       5         6        7         8        9           10           11     12    13       14     15             16         17
// Symbol ExpirationDate AskPrice AskSize BidPrice BidSize LastPrice PutCall StrikePrice Volume ImpliedVolatility Delta  Gamma  Vega,    Rho OpenInterest UnderlyingPrice DataDate
// AAPL   2013-01-04     10.55            10.35            10.55     call    540         14292  0.295             0.7809 2.4778 11.9371      8666         549.03          2013-01-02
// Symbol,ExpirationDate,AskPrice,AskSize,BidPrice,BidSize,LastPrice,PutCall,StrikePrice,Volume,ImpliedVolatility,Delta,Gamma,Vega,Rho,OpenInterest,UnderlyingPrice,DataDate
// AAPL,2013-01-04,10.55,,10.35,,10.55,call,540,14292,0.295,0.7809,2.4778,11.9371,,8666,549.03,2013-01-02
impl DataFeed for DiscountOptionData {
	fn next_tick(&mut self) -> Option<Tick> {
		let e = self.enumerator.next();

		if e.is_none() {
			return None;
		}

		let res = e.unwrap();

		if res.is_err() {
			panic!("bad result: {}", res.err().unwrap());
		}

		if ! res.is_ok() {
			println!("we are not okay!");
		}

		let l = res.unwrap();

		let v: Vec<&str> = l.split(',').collect();
		assert_eq!(v.len(), 18);

		// println!("length: {}", v.len());

		let symbol: String = v[0].parse().unwrap();

		let expiration_date_data: String = v[1].parse().unwrap();
		let expiration_date_data_str: &str = &*(expiration_date_data + TIME_TIMEZONE);
		// println!("expiration_date_data_str: {}", expiration_date_data_str);
		let expiration_date: DateTime<FixedOffset> = DateTime::parse_from_str(expiration_date_data_str, CHRONO_FORMAT).unwrap();

		let ask: f64 = v[2].parse().unwrap();
		let bid: f64 = v[4].parse().unwrap();
		let last_price: f64 = v[6].parse().unwrap();
		let call: String = v[7].parse().unwrap();
		let strike_price: f64 = v[8].parse().unwrap();
		let volume: i32 = v[9].parse().unwrap();
		let implied_volatility: f64 = v[10].parse().unwrap();
		let delta: f64 = v[11].parse().unwrap();
		let gamma: f64 = v[12].parse().unwrap();
		let vega: f64 = v[13].parse().unwrap();
		let open_interest: i32 = v[15].parse().unwrap();
		let underlying_price: f64 = v[16].parse().unwrap();

		let date_data: String = v[17].parse().unwrap();
		let date_data_str: &str = &*(date_data + TIME_TIMEZONE);
		// println!("date_data_str: {}", date_data_str);
		let date: DateTime<FixedOffset> = DateTime::parse_from_str(date_data_str, CHRONO_FORMAT).unwrap();

		let t = Tick::new(
			symbol,
			expiration_date,
			ask,
			bid,
			last_price,
			call == "call",
			strike_price,
			volume,
			implied_volatility,
			delta,
			gamma,
			vega,
			open_interest,
			underlying_price,
			date,
		);

		Some(t)
	}
}