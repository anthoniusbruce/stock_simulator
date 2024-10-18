# Stock Simulator

**Stock Simulator** is a Rust-based command-line application that performs Monte Carlo simulations on historical stock data to predict future stock performance. It takes historical gains and losses for a set of stocks and runs simulations to predict their behavior over a specified number of days. The results are output as an HTML file listing the top predicted earners.

## Features

- Reads historical stock data from CSV files in a source directory.
- Runs Monte Carlo simulations to predict stock performance.
- Supports configurable simulation parameters such as the number of days, number of simulations, and how many top stocks to output.
- Outputs an HTML file listing the top X predicted earners over the given time period.

## Usage

```bash
stock_simulator -d <days> -l <log-file> -n <number-of-simulations> -o <output-file> -s <source-dir> -t <top-x>
```

## Example
``` bash
stock_simulator -d 30 -l simulator.log -n 10000 -o predictions.html -s ./data/ -t 10
```
In this example:
- -d 30; predicts stock behavior for the next 30 days.
- -l simulator.log; specifies the log file for the application.
- -n 10000; runs 10,000 Monte Carlo simulations.
- -o predictions.html; generates an HTML file named predictions.html with the results.
- -s ./data/; specifies the directory where the input stock data (CSV files) is stored.
- -t 10; outputs the top 10 predicted earners based on the simulations.

## Input Files

The stock data is expected to be in CSV files located in the specified <source-dir>. Each file should be named after the stock symbol it represents (e.g., AAPL, MSFT) and contain historical gains or losses for that stock.

## Output

The results of the simulation are written to the specified HTML file. The output lists the top X number of stocks predicted to perform the best based on the Monte Carlo simulation.

## Log File

The log file records application logs, including any errors encountered during execution.

## Installation

Ensure you have Rust installed. Then, clone this repository and run the following command to build the application:
```bash
cargo build --release
```
You can then run the executable from the target/release directory.
## License

This project is licensed under the MIT License.
