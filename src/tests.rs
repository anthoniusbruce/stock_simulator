#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fmt::Debug;
    use std::path::PathBuf;

    use crate::monte_carlo::simulations::{
        get_percentiles, perform_simulation_calculation, simulate_period, Percentiles, Prediction,
    };
    use crate::stock_simulation::stock_simulator::{
        get_highest_x, get_simulation_data, get_thresholds, HighestLow, MostCommonResult,
        Thresholds, TopPredictions, TotalSpan, WeightedSpan,
    };

    fn vectors_are_equal<T: PartialEq + Debug>(v1: Vec<T>, v2: Vec<T>) -> bool {
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
                println!("v2 search found no {:?}", s);
                return false;
            }
        }

        for s in v2.iter() {
            if !v1.contains(&s) {
                println!("v1 search found no {:?}", s);
                return false;
            }
        }

        return true;
    }

    fn vector_is_subset<T: PartialEq + Debug>(subset: Vec<T>, original: Vec<T>) -> bool {
        for item in subset.iter() {
            if !original.contains(item) {
                println!("item {:?} not found", item);
                return false;
            }
        }

        return true;
    }

    //#[test]
    // fn build_html_page() {
    //     // assign
    //     let today = chrono::Local::now().format("%B %d, %Y");
    //     let input = vec![
    //         TopPredictions {
    //             symbol: "AACG".to_string(),
    //             most_common: 9,
    //             highest_low: -6,
    //             total_span: 33,
    //             weighted_span: -1,
    //         },
    //         TopPredictions {
    //             symbol: "AAON".to_string(),
    //             most_common: 8,
    //             highest_low: -1,
    //             total_span: 30,
    //             weighted_span: 0,
    //         },
    //     ];

    //     let expected = format!(
    //         "<!DOCTYPE html><html><head><meta charset=\"uft-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\
    //         <title>Stock Predictions</title></head><body><h1>Stock Predictions - {}</h1>\
    //         <div class=\"items-container\">\
    //             <div class=\"items-container\">\
    //                 <div class=\"item-header\">AACG</div>\
    //                 <div class=\"info\">Most common result: <span class=\"primary green\">9</span></div>\
    //                 <div class=\"info\">Bottom 25th: <span class=\"yellow\">-6</span></div>\
    //                 <div class=\"info\">25th to 75th span: <span class=\"yellow\">33</span></div>\
    //                 <div class=\"info\">Weighted span: <span class=\"red\">-1</span></div>\
    //             </div>\
    //             <div class=\"items-container\">\
    //                 <div class=\"item-header\">AAON</div>\
    //                 <div class=\"info\">Most common result: <span class=\"primary yellow\">8</span></div>\
    //                 <div class=\"info\">Bottom 25th: <span class=\"green\">-1</span></div>\
    //                 <div class=\"info\">25th to 75th span: <span class=\"green\">30</span></div>\
    //                 <div class=\"info\">Weighted span: <span class=\"yellow\">0</span></div>\
    //             </div>\
    //         </div></body></html>",
    //         today
    //     );

    //     // act
    //     let actual = get_html(&input);

    //     // assert
    //     assert_eq!(actual, expected);
    // }

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

    #[test]
    fn get_thresholds_zero_items() {
        // assign
        let input = Vec::new();
        let expected = Thresholds {
            most_common_green: 0,
            most_common_yellow: 0,
            highest_low_green: 0,
            highest_low_yellow: 0,
            total_span_green: 0,
            total_span_yellow: 0,
        };

        // act
        let actual = get_thresholds(&input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_thresholds_one_item() {
        // assign
        let input = vec![TopPredictions {
            symbol: "AACG".to_string(),
            most_common: 9,
            highest_low: -6,
            total_span: 33,
            weighted_span: 3,
        }];
        let expected = Thresholds {
            most_common_green: 9,
            most_common_yellow: 9,
            highest_low_green: -6,
            highest_low_yellow: -6,
            total_span_green: 33,
            total_span_yellow: 33,
        };

        // act
        let actual = get_thresholds(&input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_thresholds_two_items_both_same() {
        // assign
        let input = vec![
            TopPredictions {
                symbol: "AACG".to_string(),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
            TopPredictions {
                symbol: "AAON".to_string(),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
        ];
        let expected = Thresholds {
            most_common_green: 9,
            most_common_yellow: 9,
            highest_low_green: -6,
            highest_low_yellow: -6,
            total_span_green: 33,
            total_span_yellow: 33,
        };

        // act
        let actual = get_thresholds(&input);

        // assert
        assert_eq!(expected, actual);
    }
    #[test]
    fn get_thresholds_two_items() {
        // assign
        let input = vec![
            TopPredictions {
                symbol: "AACG".to_string(),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
            TopPredictions {
                symbol: "AAON".to_string(),
                most_common: 5,
                highest_low: 2,
                total_span: 7,
                weighted_span: 1,
            },
        ];
        let expected = Thresholds {
            most_common_green: 9,
            most_common_yellow: 5,
            highest_low_green: 2,
            highest_low_yellow: -6,
            total_span_green: 7,
            total_span_yellow: 33,
        };

        // act
        let actual = get_thresholds(&input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_thresholds_happy_path() {
        // assign
        let input = vec![
            TopPredictions {
                symbol: "AACG".to_string(),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
            TopPredictions {
                symbol: "AAON".to_string(),
                most_common: 5,
                highest_low: 2,
                total_span: 7,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AADI".to_string(),
                most_common: 4,
                highest_low: -2,
                total_span: 13,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AAOI".to_string(),
                most_common: 3,
                highest_low: -9,
                total_span: 26,
                weighted_span: 2,
            },
            TopPredictions {
                symbol: "AAPB".to_string(),
                most_common: 2,
                highest_low: -2,
                total_span: 7,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: "AAPL".to_string(),
                most_common: 1,
                highest_low: -1,
                total_span: 3,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: "AAPD".to_string(),
                most_common: -1,
                highest_low: -2,
                total_span: 3,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AADR".to_string(),
                most_common: -1,
                highest_low: -2,
                total_span: 2,
                weighted_span: 0,
            },
            TopPredictions {
                symbol: "AAL".to_string(),
                most_common: -3,
                highest_low: -7,
                total_span: 8,
                weighted_span: 0,
            },
            TopPredictions {
                symbol: "AAME".to_string(),
                most_common: -6,
                highest_low: -14,
                total_span: 17,
                weighted_span: 1,
            },
        ];
        let expected = Thresholds {
            most_common_green: 4,
            most_common_yellow: -1,
            highest_low_green: -2,
            highest_low_yellow: -7,
            total_span_green: 3,
            total_span_yellow: 17,
        };

        // act
        let actual = get_thresholds(&input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_x_high_most_common_result_top_5_of_10_two_of_same_result_one_falls_off_list() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 2,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: -3,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 4,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 0,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 9,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: -6,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 5,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 3,
                    _75th: 17,
                },
            },
        ];
        let top_x = 5;
        let most_common = Box::new(MostCommonResult {});
        let expected = vec![
            TopPredictions {
                symbol: "AACG".to_string(),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
            TopPredictions {
                symbol: "AAON".to_string(),
                most_common: 5,
                highest_low: 2,
                total_span: 7,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AADI".to_string(),
                most_common: 4,
                highest_low: -2,
                total_span: 13,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AAOI".to_string(),
                most_common: 3,
                highest_low: -9,
                total_span: 26,
                weighted_span: 2,
            },
            TopPredictions {
                symbol: "AAPB".to_string(),
                most_common: 2,
                highest_low: -2,
                total_span: 7,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: "AAPL".to_string(),
                most_common: 2,
                highest_low: -1,
                total_span: 3,
                weighted_span: -3,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, most_common);

        // assert
        assert_eq!(actual.len(), 5);
        assert!(vector_is_subset(actual, expected));
    }

    #[test]
    fn get_x_high_most_common_result_top_5_of_10_all_same_result() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 2,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: 2,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 2,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 2,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: 2,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 2,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 2,
                    _75th: 17,
                },
            },
        ];
        let top_x = 5;
        let most_common = Box::new(MostCommonResult {});
        let expected = vec![
            TopPredictions {
                symbol: "AACG".to_string(),
                most_common: 2,
                highest_low: -6,
                total_span: 33,
                weighted_span: 17,
            },
            TopPredictions {
                symbol: "AAON".to_string(),
                most_common: 2,
                highest_low: 2,
                total_span: 7,
                weighted_span: 7,
            },
            TopPredictions {
                symbol: "AADI".to_string(),
                most_common: 2,
                highest_low: -2,
                total_span: 13,
                weighted_span: 5,
            },
            TopPredictions {
                symbol: "AAOI".to_string(),
                most_common: 2,
                highest_low: -9,
                total_span: 26,
                weighted_span: 4,
            },
            TopPredictions {
                symbol: "AAPB".to_string(),
                most_common: 2,
                highest_low: -2,
                total_span: 7,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: "AAPL".to_string(),
                most_common: 2,
                highest_low: -1,
                total_span: 3,
                weighted_span: -3,
            },
            TopPredictions {
                symbol: "AAPD".to_string(),
                most_common: 2,
                highest_low: 2,
                total_span: 0,
                weighted_span: 0,
            },
            TopPredictions {
                symbol: "AADR".to_string(),
                most_common: 2,
                highest_low: -2,
                total_span: 4,
                weighted_span: -4,
            },
            TopPredictions {
                symbol: "AAL".to_string(),
                most_common: 2,
                highest_low: -7,
                total_span: 10,
                weighted_span: -4,
            },
            TopPredictions {
                symbol: "AAME".to_string(),
                most_common: 2,
                highest_low: -14,
                total_span: 17,
                weighted_span: -15,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, most_common);

        // assert
        assert_eq!(actual.len(), 5);
        assert!(vector_is_subset(actual, expected));
    }

    #[test]
    fn get_x_high_most_common_result_top_11_of_10() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 1,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: -3,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 4,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 0,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 9,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: -6,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 5,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 3,
                    _75th: 17,
                },
            },
        ];
        let top_x = 11;
        let most_common = Box::new(MostCommonResult {});
        let expected = vec![
            TopPredictions {
                symbol: "AACG".to_string(),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
            TopPredictions {
                symbol: "AAON".to_string(),
                most_common: 5,
                highest_low: 2,
                total_span: 7,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AADI".to_string(),
                most_common: 4,
                highest_low: -2,
                total_span: 13,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AAOI".to_string(),
                most_common: 3,
                highest_low: -9,
                total_span: 26,
                weighted_span: 2,
            },
            TopPredictions {
                symbol: "AAPB".to_string(),
                most_common: 2,
                highest_low: -2,
                total_span: 7,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: "AAPL".to_string(),
                most_common: 1,
                highest_low: -1,
                total_span: 3,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: "AAPD".to_string(),
                most_common: -1,
                highest_low: -2,
                total_span: 3,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: "AADR".to_string(),
                most_common: -1,
                highest_low: -2,
                total_span: 2,
                weighted_span: 0,
            },
            TopPredictions {
                symbol: "AAL".to_string(),
                most_common: -3,
                highest_low: -7,
                total_span: 8,
                weighted_span: 0,
            },
            TopPredictions {
                symbol: "AAME".to_string(),
                most_common: -6,
                highest_low: -14,
                total_span: 17,
                weighted_span: 1,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, most_common);

        // assert
        assert!(vectors_are_equal(expected, actual));
    }

    #[test]
    fn get_x_high_most_common_result_top_5_of_10_most_common_weighted_span() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 1,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: -3,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 4,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 0,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 9,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: -6,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 5,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 3,
                    _75th: 17,
                },
            },
        ];
        let top_x = 5;
        let weighted_span = Box::new(WeightedSpan {});
        let expected = vec![
            TopPredictions {
                symbol: String::from("AACG"),
                weighted_span: 3,
                most_common: 9,
                highest_low: -6,
                total_span: 33,
            },
            TopPredictions {
                symbol: String::from("AAOI"),
                weighted_span: 2,
                most_common: 3,
                highest_low: -9,
                total_span: 26,
            },
            TopPredictions {
                symbol: String::from("AAON"),
                weighted_span: 1,
                most_common: 5,
                highest_low: 2,
                total_span: 7,
            },
            TopPredictions {
                symbol: String::from("AAME"),
                weighted_span: 1,
                most_common: -6,
                highest_low: -14,
                total_span: 17,
            },
            TopPredictions {
                symbol: String::from("AADI"),
                weighted_span: 1,
                most_common: 4,
                highest_low: -2,
                total_span: 13,
            },
            TopPredictions {
                symbol: String::from("AAPD"),
                weighted_span: 1,
                most_common: -1,
                highest_low: -2,
                total_span: 3,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, weighted_span);

        // assert
        assert_eq!(actual.len(), 5);
        assert!(vector_is_subset(actual, expected));
    }

    #[test]
    fn get_x_high_most_common_result_top_5_of_10_most_common_total_span() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 1,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: -3,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 4,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 0,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 9,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: -6,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 5,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 3,
                    _75th: 17,
                },
            },
        ];
        let top_x = 5;
        let total_span = Box::new(TotalSpan {});
        let expected = vec![
            TopPredictions {
                symbol: String::from("AADR"),
                total_span: 2,
                most_common: -1,
                highest_low: -2,
                weighted_span: 0,
            },
            TopPredictions {
                symbol: String::from("AAPL"),
                total_span: 3,
                most_common: 1,
                highest_low: -1,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: String::from("AAPD"),
                total_span: 3,
                most_common: -1,
                highest_low: -2,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AAON"),
                total_span: 7,
                most_common: 5,
                highest_low: 2,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AAPB"),
                total_span: 7,
                most_common: 2,
                highest_low: -2,
                weighted_span: -1,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, total_span);

        // assert
        assert!(vectors_are_equal(expected, actual));
    }

    #[test]
    fn get_x_high_most_common_result_top_5_of_10_most_common_highest_low() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 1,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: -3,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 4,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 0,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 9,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: -6,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 5,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 3,
                    _75th: 17,
                },
            },
        ];
        let top_x = 5;
        let highest_low = Box::new(HighestLow {});
        let expected = vec![
            TopPredictions {
                symbol: String::from("AAON"),
                highest_low: 2,
                most_common: 5,
                total_span: 7,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AAPL"),
                highest_low: -1,
                most_common: 1,
                total_span: 3,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: String::from("AAPB"),
                highest_low: -2,
                most_common: 2,
                total_span: 7,
                weighted_span: -1,
            },
            TopPredictions {
                symbol: String::from("AAPD"),
                highest_low: -2,
                most_common: -1,
                total_span: 3,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AADI"),
                highest_low: -2,
                most_common: 4,
                total_span: 13,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AADR"),
                highest_low: -2,
                most_common: -1,
                total_span: 2,
                weighted_span: 0,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, highest_low);

        // assert
        assert_eq!(actual.len(), 5);
        assert!(vector_is_subset(actual, expected));
    }

    #[test]
    fn get_x_high_most_common_result_top_5_of_10_most_common_most_common() {
        // assign
        let predictions = vec![
            Prediction {
                symbol: "AAPB".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 2,
                    _75th: 5,
                },
            },
            Prediction {
                symbol: "AAPL".to_string(),
                percentiles: Percentiles {
                    _25th: -1,
                    _50th: 1,
                    _75th: 2,
                },
            },
            Prediction {
                symbol: "AAL".to_string(),
                percentiles: Percentiles {
                    _25th: -7,
                    _50th: -3,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AAPD".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 1,
                },
            },
            Prediction {
                symbol: "AADI".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: 4,
                    _75th: 11,
                },
            },
            Prediction {
                symbol: "AADR".to_string(),
                percentiles: Percentiles {
                    _25th: -2,
                    _50th: -1,
                    _75th: 0,
                },
            },
            Prediction {
                symbol: "AACG".to_string(),
                percentiles: Percentiles {
                    _25th: -6,
                    _50th: 9,
                    _75th: 27,
                },
            },
            Prediction {
                symbol: "AAME".to_string(),
                percentiles: Percentiles {
                    _25th: -14,
                    _50th: -6,
                    _75th: 3,
                },
            },
            Prediction {
                symbol: "AAON".to_string(),
                percentiles: Percentiles {
                    _25th: 2,
                    _50th: 5,
                    _75th: 9,
                },
            },
            Prediction {
                symbol: "AAOI".to_string(),
                percentiles: Percentiles {
                    _25th: -9,
                    _50th: 3,
                    _75th: 17,
                },
            },
        ];
        let top_x = 5;
        let most_common = Box::new(MostCommonResult {});
        let expected = vec![
            TopPredictions {
                symbol: String::from("AACG"),
                most_common: 9,
                highest_low: -6,
                total_span: 33,
                weighted_span: 3,
            },
            TopPredictions {
                symbol: String::from("AAON"),
                most_common: 5,
                highest_low: 2,
                total_span: 7,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AADI"),
                most_common: 4,
                highest_low: -2,
                total_span: 13,
                weighted_span: 1,
            },
            TopPredictions {
                symbol: String::from("AAOI"),
                most_common: 3,
                highest_low: -9,
                total_span: 26,
                weighted_span: 2,
            },
            TopPredictions {
                symbol: String::from("AAPB"),
                most_common: 2,
                highest_low: -2,
                total_span: 7,
                weighted_span: -1,
            },
        ];

        // act
        let actual = get_highest_x(top_x, &predictions, most_common);

        // assert
        assert!(vectors_are_equal(expected, actual));
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
            _25th: 5,
            _50th: 10,
            _75th: 15,
        };

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        let actual = actual_opt.unwrap();
        assert_eq!(actual._25th, expected._25th);
        assert_eq!(actual._50th, expected._50th);
        assert_eq!(actual._75th, expected._75th);
    }

    #[test]
    fn get_percentiles_one_in_result_all_that_number() {
        // assign
        let number_of_results = 1;
        let results = BTreeMap::from([(2, 1)]);
        let expected = Percentiles {
            _25th: 2,
            _50th: 2,
            _75th: 2,
        };

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        let actual = actual_opt.unwrap();
        assert_eq!(actual._25th, expected._25th);
        assert_eq!(actual._50th, expected._50th);
        assert_eq!(actual._75th, expected._75th);
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
            _25th: -13,
            _50th: 5,
            _75th: 25,
        };

        // act
        let actual_opt = get_percentiles(&results, number_of_results);

        // assert
        let actual = actual_opt.unwrap();
        assert_eq!(actual._25th, expected._25th);
        assert_eq!(actual._50th, expected._50th);
        assert_eq!(actual._75th, expected._75th);
    }
}
