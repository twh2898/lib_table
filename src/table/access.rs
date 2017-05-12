///! Functions for accessing data within the table.

use table::Table;
use std::path::Path;

impl Table {
    /// Get the number of rows in a table. This does not include the header row.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns in a table.
    pub fn cols(&self) -> usize {
        self.cols
    }
}
