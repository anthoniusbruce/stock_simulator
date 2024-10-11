pub mod stock_simulator {
    use std::{
        error::Error,
        fmt,
        fs::{self, DirEntry},
        io::{self, ErrorKind},
        path::PathBuf,
    };

    use itertools::Itertools;

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

    pub struct MostCommonResult {}
    pub struct TotalSpan {}
    pub struct WeightedSpan {}
    pub struct HighestLow {}

    pub trait PredictionManipulation {
        fn calculation(&self, prediction: &Prediction) -> i32;
        fn compare(&self, left: &TopPredictions, right: i32) -> i8;
    }

    impl PredictionManipulation for MostCommonResult {
        fn calculation(&self, prediction: &Prediction) -> i32 {
            prediction.percentiles._50th
        }

        fn compare(&self, left: &TopPredictions, right: i32) -> i8 {
            if left.most_common < right {
                -1
            } else if left.most_common > right {
                1
            } else {
                0
            }
        }
    }

    impl PredictionManipulation for TotalSpan {
        fn calculation(&self, prediction: &Prediction) -> i32 {
            prediction.percentiles._75th - prediction.percentiles._25th
        }

        fn compare(&self, left: &TopPredictions, right: i32) -> i8 {
            if left.total_span < right {
                1
            } else if left.total_span > right {
                -1
            } else {
                0
            }
        }
    }

    impl PredictionManipulation for WeightedSpan {
        fn calculation(&self, prediction: &Prediction) -> i32 {
            prediction.percentiles._75th + prediction.percentiles._25th
                - (2 * prediction.percentiles._50th)
        }

        fn compare(&self, left: &TopPredictions, right: i32) -> i8 {
            if left.weighted_span < right {
                -1
            } else if left.weighted_span > right {
                1
            } else {
                0
            }
        }
    }

    impl PredictionManipulation for HighestLow {
        fn calculation(&self, prediction: &Prediction) -> i32 {
            prediction.percentiles._25th
        }

        fn compare(&self, left: &TopPredictions, right: i32) -> i8 {
            if left.highest_low < right {
                -1
            } else if left.highest_low > right {
                1
            } else {
                0
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct TopPredictions {
        pub symbol: String,
        pub most_common: i32,
        pub highest_low: i32,
        pub total_span: i32,
        pub weighted_span: i32,
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

        output_results(&all_symbols);

        log("N/A", format!("processed {symbol_count} symbols"));
    }

    fn output_results(predictions: &Vec<Prediction>) {
        // instead output an html file that can been seen in a browser with all the data hardcoded
        let most_common_box = Box::new(MostCommonResult {});
        let prediction_calcs = get_highest_x(100, predictions, most_common_box);
        let html = get_html(&prediction_calcs);
        println!("{:?}", prediction_calcs);
        println!("{:?}", predictions);
    }

    pub(crate) fn get_html(calcs: &Vec<TopPredictions>) -> String {
        let (low, high) = get_thresholds(calcs);

        String::new()
    }

    pub(crate) fn get_thresholds(calcs: &Vec<TopPredictions>) -> (i32, i32) {
        // let count = calcs.len();
        // if count == 0 {
        //     return (0, 0);
        // }
        // let threshold_length = count / 3;
        // let low_index = threshold_length - 1;
        // let high_index = count - threshold_length - 1;

        // let vec_sorted: Vec<i32> = calcs.into_iter().sorted().map(|p| p.primary).collect();

        // (vec_sorted[low_index], vec_sorted[high_index])
        (0, 0)
    }

    pub(crate) fn get_highest_x(
        top_x: usize,
        all: &Vec<Prediction>,
        primary_filter: Box<dyn PredictionManipulation>,
    ) -> Vec<TopPredictions> {
        let mut results: Vec<TopPredictions> = Vec::new();

        for prediction in all {
            let mut index = 0;
            let primary_calc = primary_filter.calculation(prediction);
            while index < results.len() && primary_filter.compare(&results[index], primary_calc) > 0
            {
                index += 1;
            }

            let calculations = TopPredictions {
                symbol: prediction.symbol.clone(),
                most_common: MostCommonResult {}.calculation(prediction),
                highest_low: HighestLow {}.calculation(prediction),
                total_span: TotalSpan {}.calculation(prediction),
                weighted_span: WeightedSpan {}.calculation(prediction),
            };

            if index == results.len() {
                results.push(calculations);
            } else {
                results.insert(index, calculations);
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
