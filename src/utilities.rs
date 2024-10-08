pub mod util {
    use std::{fs::OpenOptions, io::Write};

    use chrono::Utc;

    use crate::LOG_FILE_PATH;

    /// convenience function to log trouble without interrupting things
    pub fn log<T: std::fmt::Debug>(symbol: &str, info: T) {
        let timestamp = Utc::now();
        let message = format!("TS: {}: {}: {:?}\n", timestamp, symbol, info);
        LOG_FILE_PATH.with(|path| {
            let opt = path.borrow().clone();
            match opt {
                Some(log_file_path) => {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(log_file_path)
                        .unwrap();
                    file.write(message.as_bytes()).unwrap();
                }
                None => {
                    println!("{}", message);
                }
            }
        });
    }
}
