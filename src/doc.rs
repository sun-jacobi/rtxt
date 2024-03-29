use crate::{Position, Row};
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Doc {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Doc {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for content in contents.lines() {
            rows.push(Row::from(content));
        }
        Ok(Self {
            rows: rows,
            filename: Some(String::from(filename)),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    fn insert_line(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        } else if at.y == self.len() {
            self.rows.push(Row::default());
        } else {
            let new = self.rows.get_mut(at.y).unwrap().split(at.x);
            self.rows.insert(at.y + 1, new);
        }
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if c == '\n' {
            self.insert_line(at);
            return;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        if at.x == self.row(at.y).unwrap().len() && at.y < self.len() - 1 {
            let next = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
