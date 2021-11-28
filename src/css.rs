struct StyleSheet {
    rules: Vec<Rule>,
}

/// css rule, a rule is a style block, like `div.note { margin-bottom: 20px; padding: 10px; }`
struct Rule {
    /// selector lists
    selectors: Vec<Selector>,

    /// declaration lists
    declarations: Vec<Declaration>,
}

/// **Specificity** is one of the ways a rendering engine decides which style
/// overrides the other in a conflict. If a stylesheet contains two rules
/// that match an element, the rule with the matching selector of higher
/// specificity can override values from the one with lower specificity.
///
/// # Examples
///
/// ```
///  *             {}  /* a=0 b=0 c=0 d=0 -> specificity = 0,0,0,0 */
///  li            {}  /* a=0 b=0 c=0 d=1 -> specificity = 0,0,0,1 */
///  li:first-line {}  /* a=0 b=0 c=0 d=2 -> specificity = 0,0,0,2 */
///  ul li         {}  /* a=0 b=0 c=0 d=2 -> specificity = 0,0,0,2 */
///  ul ol+li      {}  /* a=0 b=0 c=0 d=3 -> specificity = 0,0,0,3 */
///  h1 + *[rel=up]{}  /* a=0 b=0 c=1 d=1 -> specificity = 0,0,1,1 */
///  ul ol li.red  {}  /* a=0 b=0 c=1 d=3 -> specificity = 0,0,1,3 */
///  li.red.level  {}  /* a=0 b=0 c=2 d=1 -> specificity = 0,0,2,1 */
///  #x34y         {}  /* a=0 b=1 c=0 d=0 -> specificity = 0,1,0,0 */
///  style=""          /* a=1 b=0 c=0 d=0 -> specificity = 1,0,0,0 */
/// <HEAD>
/// <STYLE type="text/css">
///   #x97z { color: red }
/// </STYLE>
/// </HEAD>
/// <BODY>
/// <P ID=x97z style="color: green">
/// </BODY>
/// ```
///
/// above example is from `https://www.w3.org/TR/CSS2/cascade.html#specificity`,
/// but here for simple purpose, **We use three number to decide the specificity.**
///
pub type Specificity = (usize, usize, usize);

/// selector of css, like a tag name、a class name prefixed by '.'、'*'
enum Selector {
    Simple(SimpleSelector),
}

impl Selector {
    /// get the specificity of a selector
    fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

/// simple selector without any combinator
struct SimpleSelector {
    /// tag name, like `div`
    tag_name: Option<String>,

    /// id, like `#dog`
    id: Option<String>,

    /// class name, like `.apple`
    class: Vec<String>,
}

/// a name/value pair, just a style unit. for example, "margin: auto;" is a declaration.
struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn valid_identifier_char(identifier: char) -> bool {
    todo!()
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

    fn parse_rules(&mut self) -> Vec<Rule> {
        todo!()
    }

    /// parse a rule set: `<selector> { <declarations> }`
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    /// parse a comma-separated list of selectors
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                // start of declarations
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        // return selectors with highest specificity first, for use in matching
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }

    /// parse a list of declarations enclosed in `{ ... }`
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        return declarations;
    }

    /// parse one simple selector, e.g.: `type#id.class1.class2.class3`
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                // if `c` is true for method `valid_identifier_char`, use this arm
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        return selector;
    }

    /// parse one `<property>: <value>` declaration
    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        Declaration {
            name: property_name,
            value: value,
        }
    }

    fn parse_identifier(&mut self) -> String {
        todo!()
    }

    fn parse_value(&mut self) -> Value {
        todo!()
    }

    // ----------------------
    // --- utils function ---
    // ----------------------

    /// return next character
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
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
        self.consume_until(char::is_whitespace);
    }
}
