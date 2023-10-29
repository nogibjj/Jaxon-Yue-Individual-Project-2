use rusqlite::{Connection, params};
use std::error::Error;

// Establish a connection
pub fn establish_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open("wages.db")
}

// CREATE: Insert a new country's data
pub fn create_wages_data(conn: &Connection, country: &str, year_2000: f64, year_2010: f64, year_2020: f64, year_2022: f64) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO wages (country, year_2000, year_2010, year_2020, year_2022) VALUES (?, ?, ?, ?, ?)",
        params![country, year_2000, year_2010, year_2020, year_2022],
    )?;
    println!("Wages data created: {}, {}, {}, {}, {}", country, year_2000, year_2010, year_2020, year_2022);
    Ok(())
}

// // READ: Get all data
// pub fn read_all_wages_data(conn: &Connection) -> Result<Vec<(String, f64, f64, f64, f64)>, Box<dyn Error>> {
//     let mut stmt = conn.prepare("SELECT * FROM wages")?;
//     let rows = stmt.query_map([], |row| {
//         Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
//     })?;

//     let mut all_data = Vec::new();
//     for row in rows {
//         all_data.push(row?);
//     }
//     Ok(all_data)
// }

// READ: Get data by country
pub fn read_wages_data_by_country(conn: &Connection, country_name: &str) -> Result<(String, f64, f64, f64, f64), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT * FROM wages WHERE country=?")?;
    let data = stmt.query_row(params![country_name], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
    })?;
    println!("Wages data for {}: {:?}", country_name, data);
    Ok(data)
}

// UPDATE: Update data based on the country
pub fn update_wages_data(conn: &Connection, country: &str, year_2000: f64, year_2010: f64, year_2020: f64, year_2022: f64) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "UPDATE wages SET year_2000=?, year_2010=?, year_2020=?, year_2022=? WHERE country=?", 
        params![year_2000, year_2010, year_2020, year_2022, country],
    )?;
    println!("Wages data for {} successfully updated to: {}, {}, {}, {}", country, year_2000, year_2010, year_2020, year_2022);
    Ok(())
}

// DELETE: Delete data by country
pub fn delete_wages_data(conn: &Connection, country_name: &str) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM wages WHERE country=?", params![country_name])?;
    println!("Wages data for {} deleted", country_name);
    Ok(())
}

// Close the connection