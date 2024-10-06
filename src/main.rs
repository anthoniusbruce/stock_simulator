use core::cell::RefCell;
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::Utc;
use rand::Rng;

mod tests;

thread_local! {static LOG_FILE_PATH:RefCell<Option<PathBuf>> = RefCell::new(None::<PathBuf>)}

fn main() {
    let symbol = "AACG";
    let path = PathBuf::from(symbol);
    let periods = 10;
    let number_of_simulations = 1_000_000;

    let data_result = get_simulation_data(&path);
    match data_result {
        Ok(data) => {
            monte_carlo_simulation(&data, periods, number_of_simulations);
        }
        Err(e) => log(symbol, e),
    }
}
fn monte_carlo_simulation(data: &Vec<f64>, periods: u32, number_of_simulations: u32) {
    let mut results = HashMap::new();

    for _ in 1..number_of_simulations {
        let simulation = simulate_period(&data, periods);
        let calc = perform_simulation_calculation(&simulation);

        *results.entry(calc).or_insert(0) += 1;
    }

    output_results(results);
}

fn output_results<T: std::fmt::Debug>(results: T) {
    println!("results: {:?}", results);
}

fn simulate_period(input: &Vec<f64>, number_of_periods: u32) -> Vec<f64> {
    let mut ret = Vec::new();
    let mut rng = rand::thread_rng();
    let count = input.len();
    if count == 0 {
        return ret;
    }
    for _index in 0..number_of_periods {
        let rnd_index = rng.gen_range(0..count);
        ret.push(input[rnd_index]);
    }
    ret
}

fn perform_simulation_calculation(rates: &Vec<f64>) -> i32 {
    let base_investment = 100.0;
    let mut investment = base_investment;

    for rate in rates {
        investment += investment * rate;
    }

    (investment - base_investment).round() as i32
}

/// Method to get the simulation data from the comman separated file passed in to the method
fn get_simulation_data(path: &PathBuf) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut ret: Vec<f64> = Vec::new();
    let content = fs::read_to_string(path)?;
    let content = content.trim().trim_matches(',');

    let items = content.split(',');

    for item in items {
        let val = item.parse::<f64>()?;
        ret.push(val);
    }

    Ok(ret)
}

/// convenience function to log trouble without interrupting things
fn log<T: std::fmt::Debug>(symbol: &str, info: T) {
    let timestamp = Utc::now();
    let message = format!("TS: {}: {}: {:?}\n", timestamp, symbol, info);
    LOG_FILE_PATH.with(|path| {
        let opt = path.borrow().clone();
        match opt {
            Some(log_file_path) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(log_file_path)
                    .unwrap();
                file.write(message.as_bytes()).unwrap();
            }
            None => {
                println!("{}", message);
            }
        }
    });
}
