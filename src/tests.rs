#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{get_simulation_data, perform_simulation_calculation};

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

    #[test]
    fn perform_simulation_calculation_happy_path() {
        // assign
        let rates = vec![1.0, 0.5, 0.25, 0.10, -0.75];
        let expected = 3.125;

        // act
        let actual = perform_simulation_calculation(&rates);

        // assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn perform_simulation_calculation_all_zeroes_is_zero() {
        // assign
        let rates = vec![0.0, 0.0, 0.0, 0.0];
        let expected = 0.0;

        // act
        let actual = perform_simulation_calculation(&rates);

        // assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_simulation_data_happy_path() {
        // assign
        let file_name = PathBuf::from("AACG");
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
