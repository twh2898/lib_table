
pub mod read;

pub struct Table {
    rows: usize,
    cols: usize,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            rows: 0,
            cols: 0,
            headers: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

#[cfg(test)]
mod tests {
    use table::Table;

    #[test]
    fn test_new() {
        let tbl = Table::new();
        assert_eq!(0, tbl.rows());
        assert_eq!(0, tbl.cols());
    }
}
