use super::text_span::TextSpan;

#[derive(Debug, Copy, Clone)]
pub struct TextLine {
    start: usize,
    end: usize,
    end_including_line_break: usize,
}

impl TextLine {
    pub(super) fn new(start: usize, end: usize, end_including_line_break: usize) -> Self {
        Self {
            start,
            end,
            end_including_line_break,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn length_including_line_break(&self) -> usize {
        self.end_including_line_break - self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.start,
            end: self.end,
        }
    }

    pub fn span_including_line_break(&self) -> TextSpan {
        TextSpan {
            start: self.start,
            end: self.end_including_line_break,
        }
    }
}
