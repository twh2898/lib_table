
pub mod read;
pub mod access;

#[derive(Debug, Clone)]
pub struct Table {
    rows: usize,
    cols: usize,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

impl Table {
    /// Create a new Table with no data, no headers, 0 rows, and 0 columns.
    ///
    /// # Example
    /// ```rust
    /// use lib_table::table::Table;
    /// let tbl = Table::new();
    ///
    /// assert_eq!(0, tbl.rows());
    /// assert_eq!(0, tbl.cols());
    /// ```
    pub fn new() -> Table {
        Table {
            rows: 0,
            cols: 0,
            headers: Vec::new(),
            data: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use table::Table;

    #[test]
    fn test_new() {
        let tbl = Table::new();

        assert!(tbl.data.is_empty());
        assert!(tbl.headers.is_empty());

        assert_eq!(0, tbl.rows());
        assert_eq!(0, tbl.cols());
    }
}
