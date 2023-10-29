use csv::Reader;

pub fn compute_percentiles(
    csv_path: &str,
    column_name: &str,
) -> Result<(f64, f64, f64), Box<dyn std::error::Error>> {
    // Read the column from the CSV
    let mut rdr = Reader::from_path(csv_path)?;
    let mut column: Vec<f64> = rdr
        .deserialize()
        .filter_map(Result::ok)
        .filter_map(|record: std::collections::HashMap<String, String>| {
            record.get(column_name).and_then(|s| s.parse().ok())
        })
        .collect();

    // Sort the column values
    column.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Compute the percentiles
    let percentile_25 = get_percentile(&column, 0.25)?;
    let percentile_50 = get_percentile(&column, 0.50)?;
    let percentile_75 = get_percentile(&column, 0.75)?;

    Ok((percentile_25, percentile_50, percentile_75))
}

pub fn get_percentile(
    sorted_data: &[f64],
    percentile: f64,
) -> Result<f64, Box<dyn std::error::Error>> {
    let idx = (percentile * (sorted_data.len() - 1) as f64).round() as usize;
    if idx < sorted_data.len() {
        Ok(sorted_data[idx])
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid percentile index",
        )))
    }
}
