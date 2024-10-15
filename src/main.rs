use core::cell::RefCell;
use std::{
    fs,
    path::{Path, PathBuf},
};

use stock_simulation::stock_simulator::run_simulator;
use structopt::StructOpt;
use utilities::util::log;

mod monte_carlo;
mod stock_simulation;
mod tests;
mod utilities;

#[derive(StructOpt)]
#[structopt(
    name = "stock_simulator",
    about = "reads historical stock data from the supplied source directory, performs passed number of simulations for the passed number of days and outputs the predictions of the top symbols to the output directory in html form."
)]
struct Opt {
    /// input file
    #[structopt(short, parse(from_os_str), required(true))]
    source_dir: PathBuf,
    #[structopt(short, parse(from_os_str), required(true))]
    output_file: PathBuf,
    #[structopt(short, required(true))]
    days: u32,
    #[structopt(short, required(true))]
    number_of_simulations: u32,
    #[structopt(short, required(true))]
    top_x: usize,
    #[structopt(short, parse(from_os_str), required(true))]
    log_file: PathBuf,
}

thread_local! {static LOG_FILE_PATH:RefCell<Option<PathBuf>> = RefCell::new(None::<PathBuf>)}

fn main() {
    let opt = Opt::from_args_safe();
    match opt {
        Err(e) => {
            println!("{:?}", e);
            return;
        }
        Ok(args) => {
            let source_dir = args.source_dir;
            let periods = args.days;
            let number_of_simulations = args.number_of_simulations;
            let output_html = args.output_file;
            let top_x = args.top_x;
            let log_path = args.log_file;

            validate_log_file(&log_path);
            LOG_FILE_PATH.with(|path| *path.borrow_mut() = Some(log_path));

            validate_args(&source_dir, &output_html);

            run_simulator(
                &source_dir,
                periods,
                number_of_simulations,
                top_x,
                &output_html,
            );
        }
    }
}

fn validate_log_file(log_path: &PathBuf) {
    let mut log = log_path.clone();
    let mut log_exists = Path::exists(&log);
    if !log_exists {
        log.pop();
        log_exists = Path::exists(&log);
    }
    if !log_exists {
        panic!("log directory does not exist");
    }

    match fs::metadata(log) {
        Ok(md) => {
            if md.permissions().readonly() {
                panic!("you do not have permission to the log file")
            }
        }
        Err(e) => panic!("{e}"),
    }
}

fn validate_args(source_dir: &PathBuf, output_dir: &PathBuf) {
    let file_exists = Path::exists(source_dir);
    if !file_exists {
        let error = "file_name does not exist";
        log("N/A", error);
        panic!("{error}");
    }

    let mut output = output_dir.clone();
    let mut output_exists = Path::exists(&output);
    if !output_exists {
        output.pop();
        output_exists = Path::exists(&output);
    }
    if !output_exists {
        let error = "log directory does not exist";
        log("N/A", error);
        panic!("{error}");
    }

    match fs::metadata(output) {
        Ok(md) => {
            if md.permissions().readonly() {
                let error = "you do not have permission to the html file";
                log("N/A", error);
                panic!("{error}");
            }
        }
        Err(e) => {
            let error = e.to_string();
            log("N/A", error);
            panic!("{e}");
        }
    }
}
