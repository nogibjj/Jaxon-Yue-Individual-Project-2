#[cfg(test)]
mod tests {
    use project::compute_percentiles;

    #[test]
    fn test_percentiles() {
        // Define the path to your CSV file for testing.
        let csv_path = "dataset/Development of Average Annual Wages_1.csv";
        // Define the name of the column you're testing.
        let column_name = "2020";

        // Call the function to compute the percentiles
        let (p25, p50, p75) =
            compute_percentiles(csv_path, column_name).expect("Failed to compute percentiles");

        // Define the expected values
        let expected_p25 = 33199.0;
        let expected_p50 = 48605.0;
        let expected_p75 = 60309.0;

        // Assert that the calculated percentiles match the expected values
        assert_eq!(
            p25, expected_p25,
            "Expected 25th Percentile to be {}, but got {}",
            expected_p25, p25
        );
        assert_eq!(
            p50, expected_p50,
            "Expected 50th Percentile to be {}, but got {}",
            expected_p50, p50
        );
        assert_eq!(
            p75, expected_p75,
            "Expected 75th Percentile to be {}, but got {}",
            expected_p75, p75
        );
    }
}
