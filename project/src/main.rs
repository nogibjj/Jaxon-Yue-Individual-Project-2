use project::compute_percentiles;
use std::time::Instant;

fn main() {
    // Path to the CSV file
    let csv_path = "dataset/Development of Average Annual Wages_1.csv";
    // Name of the column
    let column_name = "2020";

    // Try to compute the percentiles and handle any potential errors
    let start = Instant::now();
    match compute_percentiles(csv_path, column_name) {
        Ok((p25, p50, p75)) => {
            println!("25th Percentile: {}", p25);
            println!("50th Percentile: {}", p50);
            println!("75th Percentile: {}", p75);
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
