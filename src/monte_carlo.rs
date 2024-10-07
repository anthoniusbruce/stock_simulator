pub mod simulations {
    use std::collections::BTreeMap;

    use rand::Rng;

    pub struct Percentiles {
        pub _5th: i32,
        pub _15th: i32,
        pub _25th: i32,
        pub _50th: i32,
        pub _75th: i32,
        pub _85th: i32,
        pub _95th: i32,
    }

    /// Method that will run a number of monte carlo simulations on the data passed in for the number of periods pass in
    pub(crate) fn monte_carlo_simulation(
        data: &Vec<f64>,
        periods: u32,
        number_of_simulations: u32,
    ) {
        let mut results = BTreeMap::new();

        for _ in 1..number_of_simulations {
            let simulation = simulate_period(&data, periods);
            let calc = perform_simulation_calculation(&simulation);

            *results.entry(calc).or_insert(0) += 1;
        }

        output_results(results);
    }

    pub(crate) fn get_percentiles(results: &BTreeMap<i32, i32>, total: u32) -> Option<Percentiles> {
        if results.len() == 0 {
            return None;
        }

        let mut steps = Vec::new();
        steps.push(total * 95 / 100);
        steps.push(total * 85 / 100);
        steps.push(total * 75 / 100);
        steps.push(total * 50 / 100);
        steps.push(total * 25 / 100);
        steps.push(total * 15 / 100);
        steps.push(total * 5 / 100);

        let mut count: u32 = 0;
        let mut test_val_opt = steps.pop();
        let mut pcts = Vec::new();
        for k_v in results.iter() {
            if test_val_opt.is_none() {
                break;
            }

            count += *k_v.1 as u32;
            while !test_val_opt.is_none() && Some(count) >= test_val_opt {
                pcts.push(k_v.0);
                test_val_opt = steps.pop();
            }
        }

        Some(Percentiles {
            _5th: *pcts[0],
            _15th: *pcts[1],
            _25th: *pcts[2],
            _50th: *pcts[3],
            _75th: *pcts[4],
            _85th: *pcts[5],
            _95th: *pcts[6],
        })
    }

    fn output_results<T: std::fmt::Debug>(results: T) {
        println!("results: {:?}", results);
    }

    // Method that randomly chooses period results from the input data in preparation for a simulation calculation
    pub(crate) fn simulate_period(input: &Vec<f64>, number_of_periods: u32) -> Vec<f64> {
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

    // Method that returns the result from 1 simulation
    pub(crate) fn perform_simulation_calculation(rates: &Vec<f64>) -> i32 {
        let base_investment = 100.0;
        let mut investment = base_investment;

        for rate in rates {
            investment += investment * rate;
        }

        (investment - base_investment).round() as i32
    }
}
