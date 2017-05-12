///! Functions for generateing a table from different sources.

use Table;
use std::path::Path;

impl Table {
    /// Generate a new Table from two std::vec::Vec
    pub fn from_vec(headers: Vec<String>, data: Vec<Vec<String>>) -> Result<Table, &'static str> {
        // if the header Vec is empty
        if headers.is_empty() {
            return Err("Header length must not be 0");
        }

        // if the data vec is empty
        if data.is_empty() {
            return Err("Data length must not be 0");
        }

        // check each row for length
        for row in data.iter() {
            // if the number of columns in the row do not match the number of columns in the header
            if headers.len() != row.len() {
                return Err("Malformed data: row length does not match header length");
            }
        }

        Ok(Table {
               rows: data.len(),
               cols: headers.len(),
               headers: headers,
               data: data,
           })
    }

    /// Generate a new Table from a file pointed to by path.
    pub fn from_file(path: &Path) -> Result<Table, &'static str> {
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::fs::File;

        // Check if file exists and can be read frome
        let f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err("Could not open file"),
        };

        // Get a new BufReader and read the files lines
        let reader = BufReader::new(f);
        let mut lines = reader.lines().peekable();

        // check if there is no data
        if lines.peek().is_none() {
            return Ok(Table::new());
        }

        // decode the headers from the first line
        let headers: Vec<String> = lines
            .next()
            .unwrap_or(Ok(String::new()))
            .unwrap_or(String::new())
            .trim()
            .split("\t")
            .map(|s| s.to_string())
            .collect();

        // decode the data from the remaining lines
        let data: Vec<Vec<String>> = lines
            .map(|s| {
                     s.unwrap()
                         .trim()
                         .split("\t")
                         .map(|ss| ss.to_string())
                         .collect()
                 })
            .collect();

        Ok(Table {
               rows: data.len(),
               cols: headers.len(),
               headers: headers,
               data: data,
           })
    }
}
