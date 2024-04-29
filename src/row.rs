use crate::SearchDirection;
use std::cmp;
use termion::color;
use crate::hightlighting;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    pub string: String,
    hightlighting: Vec<hightlighting::Type>,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            hightlighting: Vec::new(),
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
        
        let mut current_hightlighting = &hightlighting::Type::None;

        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            // result.push_str(grapheme)
            // // if grapheme == "\t" {
            // //     result.push(' ');
            // // } else {
            // //     result.push_str(grapheme);
            // // }
            if let Some(c) = grapheme.chars().next() {
                
                let hightlighting_type = self.hightlighting.get(index).unwrap_or(&hightlighting::Type::None);
                if hightlighting_type != current_hightlighting {
                    current_hightlighting = hightlighting_type;
                    let start_highlight = format!("{}", termion::color::Fg(hightlighting_type.to_color()));
                    result.push_str(&start_highlight[..]);

                }

                if c == '\t' {
                    result.push_str(" ")
                // } else if c.is_ascii_digit() {
                //     result.push_str(
                //         &format!(
                //             "{}{}{}",
                //             termion::color::Fg(color::Rgb(220,163,163)),
                //             c,
                //             color::Fg(color::Reset)
                //         )[..],
                //     );
                } else {
                    result.push(c);
                }
            }
        }
        let end_highlight = format!("{}", termion::color::Fg(color::Reset));
        result.push_str((&end_highlight[..]));
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
            hightlighting: Vec::new(),
            len: length_row,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn hightlight(&mut self, word: Option<&str>) {
        // let mut hightlighting = Vec::new();
        
        // for c in self.string.chars() {
        //     if c.is_ascii_digit() {
        //         hightlighting.push(hightlighting::Type::Number);
        //     } else {
        //         hightlighting.push(hightlighting::Type::None);
        //     }
        // }
        // self.hightlighting = hightlighting;

        let mut highlighting = Vec::new();            
        let chars: Vec<char> = self.string.chars().collect();            
        let mut matches = Vec::new();            
        let mut search_index = 0;            

        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index, SearchDirection::Forward) {            
                matches.push(search_match);
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())            
                {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }

        let mut index = 0;
        while let Some(c) = chars.get(index) {
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word[..].graphemes(true) {
                        index += 1;
                        highlighting.push(hightlighting::Type::Match);
                    }
                    continue;
                }
            }

            if c.is_ascii_digit() {
                highlighting.push(hightlighting::Type::Number);
            } else {
                highlighting.push(hightlighting::Type::None);
            }
            index += 1;
        }

        self.hightlighting = highlighting;

        // let mut hightlighting = Vec::new();

        // let chars: Vec<char> = self.string.chars().collect();
        // let mut matchs = Vec::new();
        // let mut search_index = 0;

        // if let Some(word) = word {
        //     while let Some(search_match) = self.find(word, search_index, SearchDirection::Forward) {
        //         matchs.push(search_match);
        //         if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count()) {
        //             search_index = next_index;
        //         } else {
        //             break;
        //         }
        //     }
        // }

        // let mut index = 0;
        // while let Some(c) = chars.get(index) {
        //     if let Some(word) = word {
        //         if matchs.contains(&index) {
        //             for _ in word[..].graphemes(true) {
        //                 index += 1;
        //                 hightlighting.push(hightlighting::Type::Match);
        //             }
        //             continue;
        //         }
        //     }
        //     if c.is_ascii_digit() {
        //         hightlighting.push(hightlighting::Type::Number);
        //     } else {
        //         hightlighting.push(hightlighting::Type::None);
        //     }
        // }
        // self.hightlighting = hightlighting;

    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };
        let sub_string: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            sub_string.find(query)
        } else {
            sub_string.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in sub_string.grapheme_indices(true).enumerate() {
                if matching_byte_index == byte_index {
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }
    // pub fn find(&self, query: &str, after: usize, direction: usize) -> Option<usize> {
    //     let sub_string: String = self.string[..].graphemes(true).skip(after).collect();
    //     let matching_byte_index = sub_string.find(query);
    //     if let Some(matching_byte_index) = matching_byte_index {
    //         for (grapheme_index, (byte_index, _)) in sub_string.grapheme_indices(true).enumerate() {
    //             if matching_byte_index == byte_index {
    //                 return Some(after + grapheme_index);
    //             }
    //         }
    //     }
    //     None
    // }

}

// bukausbrayvbvuybsuybviuybsdruybvyubvrby
