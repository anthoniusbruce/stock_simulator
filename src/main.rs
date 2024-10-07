use core::{cell::RefCell, fmt};
use std::{
    error::Error,
    fs::{self, DirEntry, OpenOptions},
    io::{self, ErrorKind, Write},
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
    let dir = PathBuf::from("test_data");
    let periods = 10;
    let number_of_simulations = 750_000;

    LOG_FILE_PATH.with(|path| *path.borrow_mut() = Some(PathBuf::from("logs/stock_simulator.log")));

    // get the path to the next file to be processed
    let mut symbol_file_opt = get_next_file(&dir);

    let mut symbol_count = 0;
    while !symbol_file_opt.is_none() {
        let symbol_file_result = symbol_file_opt.unwrap();
        let symbol_file: PathBuf;
        match symbol_file_result {
            Err(e) => {
                log("N/A", e);
                symbol_file_opt = get_next_file(&dir);

                continue;
            }
            Ok(sym) => symbol_file = sym,
        }
        let symbol = symbol_file.file_name().unwrap().to_str().unwrap();

        // run the simulation
        log(&symbol, "simulation begin");
        let data_result = get_simulation_data(&symbol_file);
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

        symbol_count += 1;
        if symbol_count > 10000 {
            break;
        };
        symbol_file_opt = get_next_file(&dir);
    }
    log("N/A", format!("processed {symbol_count} symbols"));
}

fn get_next_file(dir: &PathBuf) -> Option<Result<PathBuf, Box<dyn Error>>> {
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
            let mut first_opt = contents.next();
            let mut first: DirEntry;
            loop {
                if first_opt.is_none() {
                    return None;
                }

                let first_result = first_opt.unwrap();
                match first_result {
                    Err(e) => return Some(Err(Box::new(e))),
                    Ok(f) => first = f,
                }

                if first.path().is_dir() {
                    first_opt = contents.next();
                    continue;
                } else {
                    break;
                }
            }

            let file_to_process_result = move_file_to_archive(first);
            match file_to_process_result {
                Err(e) => return Some(Err(Box::new(e))),
                Ok(file_to_process) => return Some(Ok(file_to_process)),
            }
        }
    }
}

fn move_file_to_archive(f: DirEntry) -> Result<PathBuf, io::Error> {
    let mut archive = PathBuf::from(f.path().parent().unwrap());
    archive.push("archive");
    fs::create_dir_all(archive.as_path())?;
    archive.push(f.file_name());
    fs::rename(f.path(), archive.as_path())?;
    Ok(archive)
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
