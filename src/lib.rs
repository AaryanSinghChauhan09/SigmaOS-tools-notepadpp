#[derive(Debug, Clone, PartialEq)]
pub struct LexerToken {
    pub start: usize,
    pub length: usize,
    pub token_type: u32,
}

pub struct NotepadppBuffer {
    pub text: String,
    pub tokens: Vec<LexerToken>,
}

impl NotepadppBuffer {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), tokens: Vec::new() }
    }

    pub fn add_token(&mut self, start: usize, length: usize, token_type: u32) {
        self.tokens.push(LexerToken { start, length, token_type });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notepadpp_buffer() {
        // Arrange
        let mut buf = NotepadppBuffer::new("fn main() {}");
        // Act
        buf.add_token(0, 2, 1);
        // Assert
        assert_eq!(buf.tokens.len(), 1);
    }
}
