mod assassin;
use assassin::simulation::Simulation;
use assassin::feeds::discount_option_data::DiscountOptionData;
use assassin::models::pmcc::PMCC;
use assassin::broker::Broker;
use assassin::commission::charles_schwab::CharlesSchwab;

extern crate env_logger;

#[macro_use]
extern crate log;

extern crate greenback;
use greenback::Greenback as Money;

static INPUT_FILE: &'static str = "/Users/billrobinson/Desktop/aapl_2013.csv";

fn main() {
    env_logger::init().unwrap();

    let starting_capital = Money::new(100_000, 0);
    let feed = DiscountOptionData::new(INPUT_FILE);
    let test_model = PMCC::new();

    let commission = CharlesSchwab::new();

    let broker = Broker::new(starting_capital, Box::new(commission), Box::new(feed));

    let mut simulation = Simulation::new(Box::new(test_model), Box::new(broker));

    info!("Starting simulation with {}", starting_capital);
    simulation.run();

    simulation.print_stats();
}
