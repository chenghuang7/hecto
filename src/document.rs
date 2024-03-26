use std::{fs, io::Error};

use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    // pub fn open() -> Self {
    //     let mut rows = Vec::new();
    //     rows.push(Row::from("Hello,world"));
    //     Self { rows }
    // }
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for item in contents.lines() {
            print!("{}", item);
            rows.push(Row::from(item));
        }
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
