use core::cell::RefCell;
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::Utc;

mod tests;

thread_local! {static LOG_FILE_PATH:RefCell<Option<PathBuf>> = RefCell::new(None::<PathBuf>)}

fn main() {}

fn perform_simulation_calculation(rates: &Vec<f64>) -> f64 {
    let base_investment = 100.0;
    let mut investment = base_investment;

    for rate in rates {
        investment += investment * rate;
    }

    investment - base_investment
}

/// Method to get the simulation data from the comman separated file passed in to the method
fn get_simulation_data(path: &PathBuf) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut ret: Vec<f64> = Vec::new();
    let content = fs::read_to_string(path)?;
    let content = content.trim().trim_matches(',');

    let items = content.split(',');

    for item in items {
        println!("{}", item);
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
