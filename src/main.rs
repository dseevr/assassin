mod assassin;
use assassin::simulation::Simulation;
use assassin::feeds::discount_option_data::DiscountOptionData;
use assassin::models::pmcc::PMCC;
use assassin::brokers::basic::BasicBroker;
use assassin::commission::charles_schwab::CharlesSchwab;

static INPUT_FILE: &'static str = "/Users/billrobinson/Desktop/aapl_2013.csv";

fn main() {
	let feed = DiscountOptionData::new(INPUT_FILE);
	let test_model = PMCC::new();

	let commission = CharlesSchwab::new();
	let broker = BasicBroker::new(10_000.0, Box::new(commission), Box::new(feed));

	let mut simulation = Simulation::new(Box::new(test_model), Box::new(broker));

	simulation.run();

	println!(
		"Ran simulation ({} ticks) in {:.2} seconds",
		simulation.ticks_processed(),
		simulation.total_run_time()
	);

	// TODO: output starting capital, ending capital, profit, growth %
}
