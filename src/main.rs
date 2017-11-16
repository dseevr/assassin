use std::time::Instant;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::fs::File;

extern crate chrono;

use chrono::prelude::*;

// ----- STRUCTS ----------------------------------------------------------------

trait DataFeed {
	fn next_tick(&mut self) -> Option<Tick>;
}

// Symbol ExpirationDate AskPrice AskSize BidPrice BidSize LastPrice PutCall StrikePrice Volume ImpliedVolatility Delta  Gamma  Vega,    Rho OpenInterest UnderlyingPrice DataDate
// AAPL   2013-01-04     10.55            10.35            10.55     call    540         14292  0.295             0.7809 2.4778 11.9371      8666         549.03          2013-01-02
// AAPL,2013-01-04,10.55,,10.35,,10.55,call,540,14292,0.295,0.7809,2.4778,11.9371,,8666,549.03,2013-01-02
#[allow(dead_code)]
struct Tick {
	symbol: String,
	expiration_date: DateTime<FixedOffset>,
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
	date: DateTime<FixedOffset>,
}

trait Model {
	fn get_name(&self) -> &'static str;
	fn process_tick(&mut self, Tick);
}

struct Simulation {
	feed: Box<DataFeed>,
	model: Box<Model>,
	// TODO: add settings variables (slippage, spread multipliers, etc.)
	// TODO: add target stats that the model must hit (sharpe, DD, etc.)

	ticks_processed: i64,

	start_time: Instant,
}

impl Simulation {
	fn new(model: Box<Model>, feed: Box<DataFeed>) -> Simulation {
		Simulation {
			feed: feed,
			model: model,
			ticks_processed: 0,
			start_time: Instant::now(),
		}
	}

	fn run(&mut self) {
		println!("running simulation");

		while let Some(tick) = self.feed.next_tick() {
			self.model.process_tick(tick);
			self.ticks_processed += 1
		}

		println!("simulation finished");
		println!("processed {} ticks", self.ticks_processed);
	}

	fn total_run_time(&self) -> f64 {
		let seconds = self.start_time.elapsed().as_secs() as f64;
		let nanoseconds = self.start_time.elapsed().subsec_nanos() as f64 * 1e-9;
		return seconds + nanoseconds;
	}
}

// ----- TRADING MODELS ---------------------------------------------------------

struct DummyModel {}

impl DummyModel {
	fn new() -> DummyModel {
		DummyModel{}
	}
}

impl Model for DummyModel {
	fn get_name(&self) -> &'static str {
		"dummy model"
	}

	fn process_tick(&mut self, _tick: Tick) {
		// println!("spread: {}", tick.ask - tick.bid);
	}
}

// ----- CUSTOM FEEDS -----------------------------------------------------------

struct OptionFeed {
	enumerator: Lines<BufReader<File>>,
}

impl OptionFeed {
	fn new(filename: &'static str) -> OptionFeed {
		let file = BufReader::new(File::open(filename).unwrap());
		let enumerator = file.lines();

		// eat the header row
		// enumerator.next();

		OptionFeed{
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
impl DataFeed for OptionFeed {
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

		let t = Tick{
			symbol: symbol,
			expiration_date: expiration_date,
			ask: ask,
			bid: bid,
			last_price: last_price,
			call: call == "call",
			strike_price: strike_price,
			volume: volume,
			implied_volatility: implied_volatility,
			delta: delta,
			gamma: gamma,
			vega: vega,
			open_interest: open_interest,
			underlying_price: underlying_price,
			date: date,
		};

		Some(t)
	}
}

// ----- ENTRYPOINT -------------------------------------------------------------

static INPUT_FILE: &'static str = "/Users/billrobinson/Desktop/aapl.csv";

fn main() {
	let base_feed = Box::new(OptionFeed::new(INPUT_FILE));
	let test_model = Box::new(DummyModel::new());

	let mut simulation = Simulation::new(test_model, base_feed);

	simulation.run();

	println!("Ran simulation ({} ticks) in {:.2} seconds", simulation.ticks_processed, simulation.total_run_time());
}
