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
            len: slice.graphemes(true).count(),
        };
        // row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        // self.string.get(start..end).unwrap_or_default().to_string()
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            // result.push_str(grapheme)
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        // self.string.len()
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        // } else {
        let mut result = String::new();
        let mut length = 0;
        for (idx, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if idx == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at > self.len() {
            return;
        }
        let mut result = String::new();
        let mut length = 0;
        for (idx, grapheme) in self.string[..].graphemes(true).enumerate() {
            if idx != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut result_s = String::new();
        let mut length_s = 0;
        let mut split_row = String::new();
        let mut length_row = 0;
        for (idx, grapheme) in self.string[..].graphemes(true).enumerate() {
            if idx < at {
                length_s += 1;
                result_s.push_str(grapheme);
            } else {
                length_row += 1;
                split_row.push_str(grapheme);
            }
        }
        self.len = length_s;
        self.string = result_s;

        Self {
            string: split_row,
            len: length_row,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn find(&self, query: &str) -> Option<usize> {
        let matching_byte_index = self.string.find(query);
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                self.string[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(grapheme_index);
                }
            }
        }
        None
    }
}

// bukausbrayvbvuybsuybviuybsdruybvyubvrby
