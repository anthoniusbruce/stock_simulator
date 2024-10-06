pub mod simulations {
    use std::collections::BTreeMap;

    use rand::Rng;

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
