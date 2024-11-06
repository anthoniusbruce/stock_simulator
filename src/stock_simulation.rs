pub mod stock_simulator {
    use std::{
        error::Error,
        fmt,
        fs::{self, DirEntry, File},
        io::{self, ErrorKind, Write},
        path::PathBuf,
        vec,
    };

    use build_html::{Container, ContainerType, Html, HtmlContainer, HtmlPage};
    use itertools::Itertools;

    use crate::{
        monte_carlo::simulations::{self, Prediction},
        utilities::util::log,
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

    #[derive(PartialEq, Debug)]
    pub struct Thresholds {
        pub most_common_green: i32,
        pub most_common_yellow: i32,
        pub highest_low_green: i32,
        pub highest_low_yellow: i32,
        pub total_span_green: i32,
        pub total_span_yellow: i32,
    }

    pub fn run_simulator(
        dir: &PathBuf,
        periods: u32,
        number_of_simulations: u32,
        top_x: usize,
        output_html: &PathBuf,
    ) {
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

        output_results(top_x, output_html, &all_symbols);

        log("N/A", format!("processed {symbol_count} symbols"));
    }

    fn output_results(top_x: usize, output_html: &PathBuf, predictions: &Vec<Prediction>) {
        // instead output an html file that can been seen in a browser with all the data hardcoded
        let most_common_box = Box::new(MostCommonResult {});
        log("N/A", "determine top x begin");
        let prediction_calcs = get_highest_x(top_x, predictions, most_common_box);
        log("N/A", "determine top x end");
        log("N/A", "html creation begin");
        let html = get_html(&prediction_calcs);
        log("N/A", "html creation end");
        save_results(output_html, &html);
    }

    fn save_results(path: &PathBuf, html: &str) {
        let file_result = File::create(path);
        match file_result {
            Err(e) => log("N/A", e),
            Ok(mut file) => match file.write_all(html.as_bytes()) {
                Err(e) => log("N/A", e),
                Ok(_) => (),
            },
        }
    }

    pub(crate) fn get_html(calcs: &Vec<TopPredictions>) -> String {
        let threholds = get_thresholds(calcs);

        let mut list =
            Container::new(ContainerType::Div).with_attributes(vec![("class", "items-container")]);

        for pred in calcs {
            let mut outer_div = Container::new(ContainerType::Div)
                .with_attributes(vec![("class", "item-container")]);

            let mut title_div = Container::new(ContainerType::Div)
                .with_attributes(vec![("class", "item-header"), ("id", &*pred.symbol)]);
            title_div.add_html(&*pred.symbol);
            outer_div.add_container(title_div);

            // Most Common
            let mut most_common =
                Container::new(ContainerType::Div).with_attributes(vec![("class", "info")]);
            let color: &str;
            if pred.most_common >= threholds.most_common_green {
                color = "green";
            } else if pred.most_common < threholds.most_common_yellow {
                color = "red";
            } else {
                color = "yellow";
            }
            most_common.add_html(format!(
                "Most common result: <span class=\"primary {}\">{}</span>",
                color, pred.most_common
            ));
            outer_div.add_container(most_common);

            // Highest Low
            let mut highest_low =
                Container::new(ContainerType::Div).with_attributes(vec![("class", "info")]);
            let color: &str;
            if pred.highest_low >= threholds.highest_low_green {
                color = "green";
            } else if pred.highest_low < threholds.highest_low_yellow {
                color = "red";
            } else {
                color = "yellow";
            }
            highest_low.add_html(format!(
                "Bottom 25th: <span class=\"{}\">{}</span>",
                color, pred.highest_low
            ));
            outer_div.add_container(highest_low);

            // Total Span
            let mut total_span =
                Container::new(ContainerType::Div).with_attributes(vec![("class", "info")]);
            let color: &str;
            if pred.total_span <= threholds.total_span_green {
                color = "green";
            } else if pred.total_span > threholds.total_span_yellow {
                color = "red";
            } else {
                color = "yellow";
            }
            total_span.add_html(format!(
                "25th to 75th span: <span class=\"{}\">{}</span>",
                color, pred.total_span
            ));
            outer_div.add_container(total_span);

            // Weighted Span
            let mut weighted_span =
                Container::new(ContainerType::Div).with_attributes(vec![("class", "info")]);
            let color: &str;
            if pred.weighted_span > 0 {
                color = "green";
            } else if pred.weighted_span < 0 {
                color = "red";
            } else {
                color = "yellow";
            }
            weighted_span.add_html(format!(
                "Weighted span: <span class=\"{}\">{}</span>",
                color, pred.weighted_span
            ));
            outer_div.add_container(weighted_span);

            list.add_container(outer_div);
        }

        let page = HtmlPage::new()
            .with_meta(vec![("charset", "uft-8")])
            .with_meta(vec![
                ("name", "viewport"),
                ("content", "width=device-width, initial-scale=1.0"),
            ])
            .with_title("Stock Predictions")
            .with_style(include_str!("style.css"))
            .with_header(
                1,
                chrono::Local::now().format("Stock Predictions - %B %d, %Y"),
            )
            .with_container(list);

        page.to_html_string()
    }

    pub(crate) fn get_thresholds(calcs: &Vec<TopPredictions>) -> Thresholds {
        let count = calcs.len();
        if count == 0 {
            return Thresholds {
                most_common_green: 0,
                most_common_yellow: 0,
                highest_low_green: 0,
                highest_low_yellow: 0,
                total_span_green: 0,
                total_span_yellow: 0,
            };
        }
        let threshold_length = count / 3;
        let low_index = std::cmp::max(threshold_length, 1) - 1;
        let high_index = std::cmp::min(count - threshold_length, count - 1);

        let most_common_sorted: Vec<i32> =
            calcs.into_iter().map(|p| p.most_common).sorted().collect();
        let highest_low_sorted: Vec<i32> =
            calcs.into_iter().map(|p| p.highest_low).sorted().collect();
        let total_span_sorted: Vec<i32> =
            calcs.into_iter().map(|p| p.total_span).sorted().collect();

        let thresholds = Thresholds {
            most_common_green: most_common_sorted[high_index],
            most_common_yellow: most_common_sorted[low_index],
            highest_low_green: highest_low_sorted[high_index],
            highest_low_yellow: highest_low_sorted[low_index],
            total_span_green: total_span_sorted[low_index],
            total_span_yellow: total_span_sorted[high_index],
        };

        thresholds
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
