use crate::smart_string::SmartString;

use super::{text_line::TextLine, text_span::TextSpan};
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct SourceText {
    text: SmartString,
    lines: Vec<TextLine>,
}

impl SourceText {
    fn new_internal(text: String) -> Self {
        let text = SmartString::new(text);
        Self {
            lines: Self::parse_lines(&text),
            text,
        }
    }

    fn parse_lines(text: &SmartString) -> Vec<TextLine> {
        let mut result = vec![];
        let mut position = 0;
        let mut line_start = 0;
        while position < text.len() {
            let line_break_width = Self::get_line_break_width(text, position);

            if line_break_width == 0 {
                position += 1;
            } else {
                result.push(TextLine::new(
                    line_start,
                    position,
                    position + line_break_width,
                ));
                position += line_break_width;
                line_start = position;
            }
        }

        if position >= line_start {
            result.push(TextLine::new(line_start, position, position));
        }
        result
    }

    fn get_line_break_width(text: &SmartString, i: usize) -> usize {
        let c = text[i];
        let l = match text.get(i + 1) {
            None => '\0',
            Some(c) => c,
        };

        if c == '\r' && l == '\n' {
            2
        } else if c == '\r' || c == '\n' {
            1
        } else {
            0
        }
    }

    pub(crate) fn new(text: String) -> Self {
        Self::new_internal(text)
    }

    pub fn get_line_index(&self, position: usize) -> Option<usize> {
        self.lines.iter().enumerate().find_map(|(i, l)| {
            if l.start() <= position && l.end() >= position {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn lines(&self) -> &[TextLine] {
        &self.lines
    }

    pub(crate) fn get(&self, index: usize) -> Option<char> {
        self.text.get(index)
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl Index<TextLine> for SourceText {
    type Output = [char];

    fn index(&self, index: TextLine) -> &Self::Output {
        &self.text[index.start()..index.end()]
    }
}

impl Index<TextSpan> for SourceText {
    type Output = [char];

    fn index(&self, index: TextSpan) -> &Self::Output {
        &self.text[index.start..index.end]
    }
}

impl Index<usize> for SourceText {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.text[index]
    }
}

impl ToString for SourceText {
    fn to_string(&self) -> String {
        self.text.to_string()
    }
}

impl From<String> for SourceText {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use spectral::asserting;

    use super::*;

    fn includes_last_line_helper(text: &str, expected_line_count: usize) {
        let source_text = SourceText::new(text.to_string());
        asserting("line count equal")
            .that(&source_text.lines().len())
            .is_equal_to(expected_line_count);
    }

    #[test]
    fn includes_last_line() {
        for (text, expected_line_count) in &[(".", 1), (".\r\n", 2), (".\r\n\r\n", 3)] {
            includes_last_line_helper(text, *expected_line_count);
        }
    }
}
