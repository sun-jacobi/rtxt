use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Doc {
    rows: Vec<Row>,
}

impl Doc {
    pub fn open(filename : &str) -> Result<Self, std::io::Error> {
        let contents =  fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for content in contents.lines(){
            rows.push(Row::from(content));
        }
        rows.push(Row::from("Hello, World"));
        Ok(Self { rows })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
