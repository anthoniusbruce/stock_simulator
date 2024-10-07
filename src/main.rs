use core::{cell::RefCell, fmt};
use std::{
    error::Error,
    fs::{self, DirEntry, OpenOptions},
    io::{ErrorKind, Write},
    path::PathBuf,
};

use chrono::Utc;
use monte_carlo::simulations;

mod monte_carlo;
mod tests;

thread_local! {static LOG_FILE_PATH:RefCell<Option<PathBuf>> = RefCell::new(None::<PathBuf>)}

#[derive(Debug)]
struct SimulationError {
    kind: ErrorKind,
    message: String,
}

impl Error for SimulationError {}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{}: {}", self.kind, self.message);
        write!(f, "{output}")
    }
}

fn main() {
    let symbol = "AACG";
    let dir = PathBuf::from("test_data");
    let periods = 10;
    let number_of_simulations = 750_000;

    LOG_FILE_PATH.with(|path| *path.borrow_mut() = Some(PathBuf::from("logs/stock_simulator.log")));

    // get the path to the next file to be processed
    let symbol_file_opt = get_next_file(&dir);
    if symbol_file_opt.is_none() {
        log("N/A", "no files to process");
        return;
    }
    let symbol_file_result = symbol_file_opt.unwrap();
    let symbol_file: DirEntry;
    match symbol_file_result {
        Err(e) => {
            log("N/A", e);
            return;
        }
        Ok(sym) => symbol_file = sym,
    }

    // run the simulation
    log(&symbol, "simulation begin");
    let data_result = get_simulation_data(&symbol_file.path());
    match data_result {
        Ok(data) => {
            log(
                &symbol,
                format!(
                    "{} items, {periods} periods, {number_of_simulations} simulations",
                    &data.len()
                ),
            );
            simulations::monte_carlo_simulation(&data, periods, number_of_simulations);
        }
        Err(e) => log(symbol, e),
    }
    log(&symbol, "simulation end");
}

fn get_next_file(dir: &PathBuf) -> Option<Result<DirEntry, Box<dyn Error>>> {
    if !dir.is_dir() {
        return Some(Err(Box::new(SimulationError {
            kind: ErrorKind::NotFound,
            message: format!("directory not found: {:?}", dir),
        })));
    }

    let contents_result = fs::read_dir(dir);
    match contents_result {
        Err(e) => return Some(Err(Box::new(e))),
        Ok(mut contents) => {
            let first_opt = contents.next();
            if first_opt.is_none() {
                return None;
            }

            let first_result = first_opt.unwrap();
            match first_result {
                Err(e) => return Some(Err(Box::new(e))),
                Ok(first) => {
                    if !first.path().is_file() {
                        return Some(Err(Box::new(SimulationError {
                            kind: ErrorKind::NotFound,
                            message: format!("{:?} is not a file", first.path()),
                        })));
                    }

                    Some(Ok(first))
                }
            }
        }
    }
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
