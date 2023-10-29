mod mylib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the dataset into the SQLite database
    mylib::load_data::load("Development of Average Annual Wages_1.csv")?;

    // Establish the connection for further operations
    let conn = mylib::operations::establish_connection()?;

    // Create wages data with 5-digit numbers
    mylib::operations::create_wages_data(&conn, "China", 10000.0, 15000.0, 20000.0, 22000.0)?;

    // Update wages data
    mylib::operations::update_wages_data(&conn, "Iceland", 20000.0, 25000.0, 30000.0, 32000.0)?;

    // Print wages data
    mylib::operations::read_wages_data_by_country(&conn, "China")?;

    // Delete wages data for "China"
    mylib::operations::delete_wages_data(&conn, "China")?;

    // The connection will automatically close when `conn` goes out of scope at the end of the `main` function.

    Ok(())
}
