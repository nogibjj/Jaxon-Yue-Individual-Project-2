#[cfg(test)]

mod tests {
    use mylib::{load_data, operations};

    #[test]
    fn test_load_data() {
        let result = load_data::load("Development of Average Annual Wages_1.csv");
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_and_delete_wages_data() {
        let conn = operations::establish_connection().unwrap();
        let result =
            operations::create_wages_data(&conn, "TestCountry", 1000.0, 1500.0, 2000.0, 2200.0);
        assert!(result.is_ok());

        // Delete the test data to keep the DB clean
        let delete_result = operations::delete_wages_data(&conn, "TestCountry");
        assert!(delete_result.is_ok());
    }

    #[test]
    fn test_update_wages_data() {
        let conn = operations::establish_connection().unwrap();
        // First, ensure there's data to update
        operations::create_wages_data(&conn, "TestCountry", 1000.0, 1500.0, 2000.0, 2200.0)
            .unwrap();

        let result =
            operations::update_wages_data(&conn, "TestCountry", 1100.0, 1550.0, 2050.0, 2250.0);
        assert!(result.is_ok());

        // Clean up
        operations::delete_wages_data(&conn, "TestCountry").unwrap();
    }

    #[test]
    fn test_read_wages_data_by_country() {
        let conn = operations::establish_connection().unwrap();
        // Insert test data
        operations::create_wages_data(&conn, "TestCountry", 1000.0, 1500.0, 2000.0, 2200.0)
            .unwrap();

        let result = operations::read_wages_data_by_country(&conn, "TestCountry");
        assert!(result.is_ok());

        // Clean up
        operations::delete_wages_data(&conn, "TestCountry").unwrap();
    }
}
