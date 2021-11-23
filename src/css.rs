struct StyleSheet {
    rules: Vec<Rule>,
}

/// rule of css, like `div.note { margin-bottom: 20px; padding: 10px; }`
struct Rule {
    /// selector lists
    selectors: Vec<Selector>,

    /// declaration lists
    declarations: Vec<Declaration>,
}

/// selector of css, like a tag name、a class name prefixed by '.'、'*'
enum Selector {
    Simple(SimpleSelector),
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

struct Parser {}

impl Parser {
    /// parse one simple selector, e.g.: `type#id.class1.class2.class3`
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        todo!()
    }
}
