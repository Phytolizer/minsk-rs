use std::ops::{Index, Range, RangeTo};

#[derive(Debug, Clone)]
pub struct SmartString {
    text: Vec<char>,
}

impl SmartString {
    pub fn new(text: String) -> Self {
        Self {
            text: text.chars().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn get(&self, index: usize) -> Option<char> {
        self.text.get(index).cloned()
    }
}

impl Index<usize> for SmartString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.text[index]
    }
}

impl Index<RangeTo<usize>> for SmartString {
    type Output = [char];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.text[index]
    }
}

impl Index<Range<usize>> for SmartString {
    type Output = [char];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.text[index]
    }
}

impl ToString for SmartString {
    fn to_string(&self) -> String {
        self.text.iter().collect()
    }
}
