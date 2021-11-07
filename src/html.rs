struct Parser {
    /// the index of the next character we haven't processed yet
    pos: usize,

    /// input string
    input: String,
}

impl Parser {
    /// return next character
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// return `true` if next characters start with the given string `s`
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// return `true` if all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// return the current character and advance self.pos to the next character
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        // if all input is consumed, add `1` to indicate ending of input
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    /// consume characters until `filter` function return false
    fn consume_until<F>(&mut self, filter: F) -> String
    where
        F: Fn(char) -> bool,
    {
      let mut result = String::new();
      while !self.eof() && filter(self.next_char()) {
        result.push(self.consume_char());
      }
      return result;
    }
}
