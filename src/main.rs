use core::cell::RefCell;
use std::path::PathBuf;

use stock_simulation::stock_simulator::run_simulator;

mod monte_carlo;
mod stock_simulation;
mod tests;
mod utilities;

thread_local! {static LOG_FILE_PATH:RefCell<Option<PathBuf>> = RefCell::new(None::<PathBuf>)}

fn main() {
    let dir = PathBuf::from("test_data");
    let periods = 10;
    let number_of_simulations = 750_000;
    let output_html = PathBuf::from("predictions.html");

    run_simulator(&dir, &output_html, periods, number_of_simulations);
}
