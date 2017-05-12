
use table::Table;
use std::path::Path;

impl Table {
    pub fn from_vec(headers: Vec<String>, data: Vec<Vec<String>>) -> Result<Table, &'static str> {
        if headers.is_empty() {
            return Err("Header length must not be 0");
        }

        if data.is_empty() {
            return Err("Data length must not be 0");
        }

        for row in data.iter() {
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

    pub fn from_file(path: &Path) -> Result<Table, &'static str> {
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::fs::File;

        let f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err("Could not open file"),
        };

        let reader = BufReader::new(f);
        let mut lines = reader.lines().peekable();

        if lines.peek().is_none() {
            return Err("Not enough data, empty file");
        }

        let headers: Vec<String> = lines
            .next()
            .unwrap_or(Ok(String::new()))
            .unwrap_or(String::new())
            .trim()
            .split("\t")
            .map(|s| s.to_string())
            .collect();

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
