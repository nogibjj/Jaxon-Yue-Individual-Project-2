use rusqlite::{params, Connection};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;

impl Error for CustomError {}

#[derive(Debug)]
pub enum CustomError {
    Sqlite(rusqlite::Error),
    Csv(csv::Error),
    Io(io::Error),
}

impl From<rusqlite::Error> for CustomError {
    fn from(err: rusqlite::Error) -> CustomError {
        CustomError::Sqlite(err)
    }
}

impl From<csv::Error> for CustomError {
    fn from(err: csv::Error) -> CustomError {
        CustomError::Csv(err)
    }
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Csv(ref e) => write!(f, "CSV error: {}", e),
            CustomError::Io(ref e) => write!(f, "IO error: {}", e),
            CustomError::Sqlite(ref e) => write!(f, "Sqlite error: {}", e),
        }
    }
}

pub fn load(dataset: &str) -> Result<(), CustomError> {
    // Connect to SQLite database
    let conn = Connection::open("wages.db")?;

    // Create a table named 'wages'
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wages (
             Country TEXT,
             year_2000 DOUBLE,
             year_2010 DOUBLE,
             year_2020 DOUBLE,
             year_2022 DOUBLE
         )",
        [],
    )?;

    // Read the CSV file and insert data into the SQLite database
    let mut rdr = csv::Reader::from_reader(File::open(dataset)?);

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        let country: String = record.get(0).unwrap_or_default().to_string();
        let year_2000: f64 = record.get(1).unwrap_or_default().parse().unwrap_or(0.0);
        let year_2010: f64 = record.get(2).unwrap_or_default().parse().unwrap_or(0.0);
        let year_2020: f64 = record.get(3).unwrap_or_default().parse().unwrap_or(0.0);
        let year_2022: f64 = record.get(4).unwrap_or_default().parse().unwrap_or(0.0);

        conn.execute(
            "INSERT INTO wages (Country, year_2000, year_2010, year_2020, year_2022) VALUES (?, ?, ?, ?, ?)",
            params![country, year_2000, year_2010, year_2020, year_2022],
        )?;
    }

    Ok(())
}
