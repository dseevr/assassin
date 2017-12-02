use std::time::Instant;

use assassin::broker::Broker;
use assassin::traits::*;

extern crate greenback;
use greenback::Greenback as Money;
use greenback::util::add_commas;

pub struct Simulation {
    model: Box<Model>,
    broker: Box<Broker>,

    // TODO: add settings variables (slippage, spread multipliers, etc.)
    // TODO: add target stats that the model must hit (sharpe, DD, etc.)
    start_time: Instant,
    starting_balance: Money,
}

impl Simulation {
    pub fn new(model: Box<Model>, broker: Box<Broker>) -> Simulation {
        let starting_balance = broker.account_balance();

        Simulation {
            model: model,
            broker: broker,
            start_time: Instant::now(),
            starting_balance: starting_balance,
        }
    }

    pub fn run(&mut self) {
        self.model.before_simulation(&mut *self.broker);

        // TODO: broker and model should communicate via a channel
        self.broker.process_simulation_data(&mut *self.model);

        self.model.after_simulation(&mut *self.broker);

        // do this after after_simulation to allow for EOD data to have a chance
        // to do something on the last day of data
        self.broker.close_all_positions();
    }

    pub fn print_stats(&self) {
        println!("");
        println!("===============================================================");
        println!("");

        let balance = self.broker.account_balance();

        println!("===== POSITIONS =====");
        println!("");

        let mut running_total = self.starting_balance;

        let positions = self.broker.positions();

        for pos in &positions {
            println!("----- {} -----", pos.name());

            for o in pos.orders() {
                running_total = running_total + o.canonical_cost_basis();

                // BUY 10 contracts @ $15
                println!(
                    "  {} {} {} contracts @ {}",
                    o.buy_or_sell_string(),
                    o.quantity(),
                    o.option_name(),
                    o.fill_price(),
                );
            }
            println!("");

            println!("Commission paid: {}", pos.commission_paid());
            println!("Position value: {}", pos.realized_profit());
            println!("Running total: {}", running_total);
            println!("");
        }

        let balance_change = balance - self.starting_balance;

        println!("===== RESULTS =====");
        println!("");
        println!("Starting balance: {}", self.starting_balance);
        println!("Ending balance: {}", balance);
        println!("Change: {}", balance_change);

        let capital_growth = ((balance.raw_value() as f32
            / self.starting_balance.raw_value() as f32) * 100.0)
            - 100.0;

        let total_commish: Money = positions.iter().map(|p| p.commission_paid()).sum();

        let commish_percent_of_profit = if balance_change > Money::zero() {
            (total_commish.raw_value() as f32 / balance_change.raw_value() as f32) * 100.0
        } else {
            0.0
        };

        let order_counts: Vec<i32> = positions.iter().map(|p| p.order_count()).collect();
        let total_order_count: i32 = order_counts.iter().sum();

        let average_commission = {
            if total_order_count > 0 {
                total_commish / total_order_count
            } else {
                Money::zero()
            }
        };

        println!("Capital growth: {:.2}%", capital_growth);
        println!("Total orders: {}", total_order_count);
        println!(
            "Commission paid: {} ({:.2}% of profit)",
            total_commish,
            commish_percent_of_profit,
        );
        println!("Average commission per order: {}", average_commission);
        println!(
            "Highest account balance: {}",
            self.broker.highest_account_balance()
        );
        println!(
            "Lowest account balance: {}",
            self.broker.lowest_account_balance()
        );
        println!("");

        let ticks_per_sec = self.broker.ticks_processed() as f32 / self.total_run_time();

        println!(
            "Ran simulation ({} ticks) in {:.2} seconds ({}/sec)",
            add_commas(self.broker.ticks_processed()),
            self.total_run_time(),
            add_commas(ticks_per_sec as i64),
        );
        println!("");
    }

    pub fn total_run_time(&self) -> f32 {
        let seconds = self.start_time.elapsed().as_secs() as f32;
        let nanoseconds = self.start_time.elapsed().subsec_nanos() as f32 * 1e-9;

        seconds + nanoseconds
    }
}
