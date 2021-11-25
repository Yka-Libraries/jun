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

struct Parser {}

impl Parser {
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

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        todo!()
    }

    fn consume_whitespace(&mut self) {
        todo!()
    }

    fn eof(&mut self) -> bool {
        todo!()
    }

    fn next_char(&mut self) -> char {
        todo!()
    }

    fn consume_char(&mut self) -> char {
        todo!()
    }

    fn parse_identifier(&mut self) -> String {
        todo!()
    }
}
