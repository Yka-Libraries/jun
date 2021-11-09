use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    /// children of a dom node
    children: Vec<Node>,

    /// type of a dom node
    node_type: NodeType,
}

enum NodeType {
    /// text node
    Text(String),

    /// element node
    Element(ElementData),
}

struct ElementData {
    /// tag name of element node,
    /// like `div`、`p`
    tag_name: String,

    /// attributes of element node, is a map like `<name, value>`
    attributes: AttrMap,
}

impl Node {
    /// create text node
    pub fn text(data: String) -> Node {
        // text node has no children
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    /// create element node
    pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}
