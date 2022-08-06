use std::cmp;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    pub string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len() as usize);
        let start = cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str("    ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut result = self.string[..].graphemes(true).take(at).collect::<String>();
            let leftover = self.string[..].graphemes(true).skip(at).collect::<String>();
            if c == '\t' {
                result.push_str(" ".repeat(4).as_str());
            } else {
                result.push(c);
            }
            result.push_str(&leftover);
            self.string = result;
        }
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result = self.string[..].graphemes(true).take(at).collect::<String>();
        let remainder = self.string[..]
            .graphemes(true)
            .skip(at + 1)
            .collect::<String>();
        result.push_str(&remainder);
        self.string = result;
        self.update_len();
    }

    pub fn append(&mut self, rhs: &Row) {
        self.string.push_str(&rhs.string);
        self.update_len();
    }
    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let cons = self.string.graphemes(true).take(at).collect::<String>();
        let tail = self.string.graphemes(true).skip(at).collect::<String>();
        let next = " ".repeat(at) + tail.as_str();
        self.string = cons;
        self.update_len();
        Self::from(next.as_str())
    }
}
