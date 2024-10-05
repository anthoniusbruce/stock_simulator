#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::get_simulation_data;

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
    fn get_simulation_data_happy_path() {
        // assign
        let file_name = PathBuf::from("AACG");
        let expected = vec![
            0.020000040531158447,
            -0.010000050067901611,
            -0.06000000238418579,
            -0.039999961853027344,
            0.019999980926513672,
            0.06999999284744263,
            -0.06999999284744263,
            0.019999980926513672,
            -0.029999971389770508,
            0.029999971389770508,
            0.019999980926513672,
            0.0,
            -0.019999980926513672,
            0.10000002384185791,
            0.050000011920928955,
            0.050000011920928955,
            -0.009999990463256836,
            -0.019999980926513672,
            0.039999961853027344,
            0.0,
            0.04999995231628418,
            -0.019999980926513672,
            -0.06000000238418568,
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
