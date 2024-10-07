#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use crate::get_simulation_data;
    use crate::monte_carlo::simulations::{
        get_percentiles, perform_simulation_calculation, simulate_period, Percentiles,
    };

    fn vectors_are_equal(v1: Vec<f64>, v2: Vec<f64>) -> bool {
        if v1.iter().count() != v2.iter().count() {
            println!(
                "counts are not equal v1={} v2 = {}",
                v1.iter().count(),
                v2.iter().count()
            );
            return false;
        }

        for s in v1.iter() {
            if !v2.contains(&s) {
                println!("v2 search found no {s}");
                return false;
            }
        }

        for s in v2.iter() {
            if !v1.contains(&s) {
                println!("v1 search found no {s}");
                return false;
            }
        }

        return true;
    }

    fn vector_is_subset(subset: Vec<f64>, original: Vec<f64>) -> bool {
        for item in subset.iter() {
            if !original.contains(item) {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn get_percentiles_empty_results_zeroes_in_percentiles() {
        // assign
        let results = BTreeMap::new();
        let number_of_results = results.len() as u32;

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        assert!(actual_opt.is_none());
    }

    #[test]
    fn get_percentiles_less_than_100_in_result() {
        // assign
        let number_of_results = 20;
        let results = BTreeMap::from([
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
            (12, 1),
            (13, 1),
            (14, 1),
            (15, 1),
            (16, 1),
            (17, 1),
            (18, 1),
            (19, 1),
            (20, 1),
        ]);
        let expected = Percentiles {
            _5th: 1,
            _15th: 3,
            _25th: 5,
            _50th: 10,
            _75th: 15,
            _85th: 17,
            _95th: 19,
        };

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        let actual = actual_opt.unwrap();
        assert_eq!(actual._5th, expected._5th);
        assert_eq!(actual._15th, expected._15th);
        assert_eq!(actual._25th, expected._25th);
        assert_eq!(actual._50th, expected._50th);
        assert_eq!(actual._75th, expected._75th);
        assert_eq!(actual._85th, expected._85th);
        assert_eq!(actual._95th, expected._95th);
    }

    #[test]
    fn get_percentiles_one_in_result_all_that_number() {
        // assign
        let number_of_results = 1;
        let results = BTreeMap::from([(2, 1)]);
        let expected = Percentiles {
            _5th: 2,
            _15th: 2,
            _25th: 2,
            _50th: 2,
            _75th: 2,
            _85th: 2,
            _95th: 2,
        };

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        let actual = actual_opt.unwrap();
        assert_eq!(actual._5th, expected._5th);
        assert_eq!(actual._15th, expected._15th);
        assert_eq!(actual._25th, expected._25th);
        assert_eq!(actual._50th, expected._50th);
        assert_eq!(actual._75th, expected._75th);
        assert_eq!(actual._85th, expected._85th);
        assert_eq!(actual._95th, expected._95th);
    }

    #[test]
    fn get_percentiles_happy_path() {
        // assign
        let number_of_results = 100;
        let results = BTreeMap::from([
            (-44, 1),
            (-43, 1),
            (-32, 1),
            (-30, 1),
            (-29, 1),
            (-27, 2),
            (-26, 2),
            (-24, 2),
            (-23, 2),
            (-20, 2),
            (-18, 2),
            (-17, 1),
            (-16, 3),
            (-15, 2),
            (-14, 1),
            (-13, 2),
            (-12, 1),
            (-9, 4),
            (-8, 3),
            (-7, 2),
            (-6, 2),
            (-5, 1),
            (-4, 3),
            (-3, 2),
            (-1, 1),
            (2, 2),
            (3, 2),
            (5, 1),
            (6, 1),
            (7, 2),
            (11, 1),
            (12, 2),
            (13, 3),
            (14, 1),
            (17, 2),
            (18, 3),
            (22, 1),
            (23, 5),
            (24, 1),
            (25, 3),
            (27, 1),
            (28, 1),
            (29, 1),
            (30, 1),
            (32, 1),
            (34, 1),
            (35, 1),
            (37, 1),
            (38, 1),
            (40, 1),
            (41, 1),
            (46, 1),
            (47, 1),
            (50, 1),
            (53, 1),
            (54, 1),
            (56, 2),
            (61, 2),
            (63, 1),
            (65, 1),
            (66, 1),
            (94, 1),
        ]);
        let expected = Percentiles {
            _5th: -29,
            _15th: -20,
            _25th: -13,
            _50th: 5,
            _75th: 25,
            _85th: 40,
            _95th: 61,
        };

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        let actual = actual_opt.unwrap();
        assert_eq!(actual._5th, expected._5th);
        assert_eq!(actual._15th, expected._15th);
        assert_eq!(actual._25th, expected._25th);
        assert_eq!(actual._50th, expected._50th);
        assert_eq!(actual._75th, expected._75th);
        assert_eq!(actual._85th, expected._85th);
        assert_eq!(actual._95th, expected._95th);
    }

    #[test]
    fn simulate_period_40_random_of_20_floats() {
        // assign
        let input = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8,
            8.9, 9.0, 10.1,
        ];
        let period = 40;

        // act
        let actual = simulate_period(&input, period);

        // assert
        assert_eq!(period, actual.len() as u32);
        assert!(vector_is_subset(actual, input));
    }

    #[test]
    fn simulate_period_vector_of_one_returns_vector_with_all_same() {
        // assign
        let input = vec![1.0];
        let period = 10;
        let expected = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

        // act
        let actual = simulate_period(&input, period);

        // assert
        assert_eq!(actual.len(), expected.len());
        assert!(vectors_are_equal(actual, expected));
    }

    #[test]
    fn simulate_period_empty_vector_returns_empty_vector() {
        // assign
        let input = Vec::new();
        let period = 10;

        // act
        let actual = simulate_period(&input, period);

        // assert
        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn simulate_period_10_random_of_20_floats() {
        // assign
        let input = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8,
            8.9, 9.0, 10.1,
        ];
        let period = 10;

        // act
        let actual = simulate_period(&input, period);

        // assert
        assert_eq!(period, actual.len() as u32);
        assert!(vector_is_subset(actual, input));
    }

    #[test]
    fn perform_simulation_calculation_round_down() {
        // assign
        let rates = vec![1.0, 0.5, 0.25, 0.10, -0.75];
        let expected = 3;

        // act
        let actual = perform_simulation_calculation(&rates);

        // assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn perform_simulation_calculation_round_up() {
        // assign
        let rates = vec![1.0, 0.5, 0.25, 0.10, -0.80];
        let expected = -18;

        // act
        let actual = perform_simulation_calculation(&rates);

        // assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn perform_simulation_calculation_spot_on() {
        // assign
        let rates = vec![1.0, 0.5, 0.25, 0.20, -0.6];
        let expected = 80;

        // act
        let actual = perform_simulation_calculation(&rates);

        // assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn perform_simulation_calculation_all_zeroes_is_zero() {
        // assign
        let rates = vec![0.0, 0.0, 0.0, 0.0];
        let expected = 0;

        // act
        let actual = perform_simulation_calculation(&rates);

        // assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_simulation_data_happy_path() {
        // assign
        let file_name = PathBuf::from("test_data/AACG");
        let expected = vec![
            0.03448282949422888,
            -0.01666674945089223,
            -0.1,
            -0.068965453455194,
            0.0370370002440479,
            0.12499998669539185,
            -0.12068964631322139,
            0.0392156496088299,
            -0.05660372265889812,
            0.059999942779541016,
            0.039999961853027344,
            0.0,
            -0.03773581510593208,
            0.18867930045286038,
            0.07936509888789085,
            0.07352942852206691,
            -0.013698616715036242,
            -0.026666641235351562,
            0.05479446686014497,
            0.0,
            0.05882347166332156,
            -0.02222220161814694,
            -0.06521739271277502,
        ];
        // act
        let actual_result = get_simulation_data(&file_name);

        // assert
        match actual_result {
            Ok(actual) => {
                assert!(vectors_are_equal(expected, actual))
            }
            Err(e) => assert!(false, "{e}"),
        }
    }
}
