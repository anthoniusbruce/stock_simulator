pub mod stock_simulator {
    use std::{
        error::Error,
        fmt,
        fs::{self, DirEntry},
        io::{self, ErrorKind},
        path::PathBuf,
    };

    use crate::{
        monte_carlo::simulations::{self, Prediction},
        utilities::util::log,
        LOG_FILE_PATH,
    };

    #[derive(Debug)]
    pub struct SimulationError {
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

    pub fn run_simulator(dir: &PathBuf, periods: u32, number_of_simulations: u32) {
        LOG_FILE_PATH
            .with(|path| *path.borrow_mut() = Some(PathBuf::from("logs/stock_simulator.log")));

        // get the path to the next file to be processed
        let mut symbol_file_opt = get_next_file(&dir);

        let mut symbol_count = 0;
        let mut all_symbols = Vec::new();
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
                    let results = simulations::monte_carlo_simulation(
                        symbol.to_string(),
                        &data,
                        periods,
                        number_of_simulations,
                    );

                    if results.is_some() {
                        let sim = results.unwrap();
                        all_symbols.push(sim);
                    }
                }
                Err(e) => log(symbol, e),
            }
            log(&symbol, "simulation end");

            symbol_count += 1;
            symbol_file_opt = get_next_file(&dir);
        }

        for prediction in all_symbols {
            output_results(&prediction);
        }

        log("N/A", format!("processed {symbol_count} symbols"));
    }

    fn output_results(prediction: &Prediction) {
        // convert prediction to json
        // save json to converted file or database or something.
        println!("{:?}", prediction);
    }

    pub(crate) fn get_x_high_most_common_result(
        top_x: usize,
        all: &Vec<Prediction>,
    ) -> Vec<(&str, i32)> {
        let mut results: Vec<(&str, i32)> = Vec::new();

        for prediction in all {
            let mut index = 0;
            while index < results.len() && results[index].1 > prediction.percentiles._50th {
                index += 1;
            }

            if index == results.len() {
                results.push((&prediction.symbol, prediction.percentiles._50th));
            } else {
                results.insert(index, (&prediction.symbol, prediction.percentiles._50th));
            }

            results.truncate(top_x);
        }

        results
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

                    let first_path = first.path();
                    if first_path.is_dir()
                        || first_path.metadata().unwrap().permissions().readonly()
                    {
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

    /// Method to get the simulation data from the comman separated file passed in to the method
    pub(crate) fn get_simulation_data(path: &PathBuf) -> Result<Vec<f64>, Box<dyn Error>> {
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

    fn move_file_to_archive(f: DirEntry) -> Result<PathBuf, io::Error> {
        let mut archive = PathBuf::from(f.path().parent().unwrap());
        archive.push("archive");
        fs::create_dir_all(archive.as_path())?;
        archive.push(f.file_name());
        fs::rename(f.path(), archive.as_path())?;
        Ok(archive)
    }
}
