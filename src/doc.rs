use crate::{Position, Row};
use std::fs;

#[derive(Default)]
pub struct Doc {
    rows: Vec<Row>,
}

impl Doc {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for content in contents.lines() {
            rows.push(Row::from(content));
        }
        Ok(Self { rows })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn insert(&mut self, at: &Position, c: char) {
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
            let next = self.rows.remove(at.y+1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next);
        } else  {
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
