#[derive(Debug, Clone)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
}

impl TextSpan {
    pub fn length(&self) -> usize {
        self.end - self.start
    }
}
