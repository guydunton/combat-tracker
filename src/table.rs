pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: vec![],
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        if row.len() != self.headers.len() {
            panic!("Mismatching row size & number of table headers");
        }

        self.rows.push(row);
    }

    pub fn print(&self) {
        let column_widths = self.get_column_widths();

        // Print the headers
        for i in 0..self.headers.len() {
            print!(
                "{text:<width$} ",
                text = self.headers[i],
                width = column_widths[i]
            );
        }
        println!();

        // Print each row
        for row in self.rows.iter() {
            for i in 0..row.len() {
                print!("{text:<width$} ", text = row[i], width = column_widths[i]);
            }
            println!();
        }
    }

    fn get_column_widths(&self) -> Vec<usize> {
        let mut column_widths: Vec<usize> = Vec::new();
        column_widths.reserve(self.headers.len());

        for i in 0..self.headers.len() {
            let max_len = self.rows.iter().map(|row| row[i].len()).max().unwrap();

            column_widths.push(max_len.max(self.headers[i].len()));
        }

        column_widths
    }
}
