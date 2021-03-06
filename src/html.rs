use crate::dom;
use std::collections::HashMap;

/// parse an html document and return the root element
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::Node::element("html".to_string(), HashMap::new(), nodes)
    }
}

struct Parser {
    /// the index of the next character we haven't processed yet
    pos: usize,

    /// input string
    input: String,
}

impl Parser {
    // --------------------------
    // --- dom parse function ---
    // --------------------------

    /// parse a sequence of sibling nodes
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    /// parse a single node
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /// parse a single element, including its opening tag, contents, and closing tag
    fn parse_element(&mut self) -> dom::Node {
        // opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // contents
        let children = self.parse_nodes();

        // closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::Node::element(tag_name, attrs, children);
    }

    /// parse a text node
    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_until(|c| c != '<'))
    }

    /// parse a tag or attribute name
    fn parse_tag_name(&mut self) -> String {
        self.consume_until(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    /// parse a list of `name="value"` pairs, separated by whitespace
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    /// parse a single `name="value"` pair
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    /// parse a quoted value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_until(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // ----------------------
    // --- utils function ---
    // ----------------------

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

    /// consume and discard whitespace characters
    fn consume_whitespace(&mut self) {
        // wow, so interesting! Please notice the `;`, so the result of consume_until won't be returned
        self.consume_until(char::is_whitespace);
    }
}
