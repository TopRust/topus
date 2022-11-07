use super::attribute::Attribute;
/// DOM node has three varient.
/// - Element node: node_name attributes child_nodes
/// - Text node: node_value is content
/// - Comment node: node_value is comment
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node {
    Element {
        node_name: String,
        attributes: Vec<Attribute>,
        child_nodes: Vec<Node>,
    },
    Text {
        node_value: String,
    },
    Comment {
        node_value: String,
    },
}

/// Create Vec<Node>

#[macro_export]
macro_rules! nodes {
    () => {
        Vec::<Node>::new()
    };
    ($node: expr, $($tail: tt)*) => {
        {
            let mut nodes = nodes!($($tail)*);
            nodes.push($node);
            nodes
        }
    };
    ($nodes: expr; $($tail: tt)*) => {
        {
            let mut nodes = nodes!($($tail)*);
            for node in $nodes {
                nodes.push(node);
            }
            nodes
        }
    };
    ($node: ident $($tail: tt)*) => {
        {
            let mut nodes = nodes!($($tail)*);
            nodes.push(element!($node));
            nodes
        }
    };
}

/// Parse attributes and elements

#[macro_export]
macro_rules! attributes_nodes {
    () => {
        (Vec::<Attribute>::new(), Vec::<Node>::new())
    };
    (=> $($tail: tt)*) => {
        (Vec::<Attribute>::new(), nodes!($($tail)*))
    };
    ($attribute: expr, $($tail: tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            attributes.push($attribute);
            (attributes, child_nodes)
        }
    };
    ($attributes: expr; $($tail: tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            for attribute in $attributes {
                attributes.push(attribute);
            }
            (attributes, child_nodes)
        }
    };
    ($key:ident = $value:expr, $($tail:tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            attributes.push(attribute!($key=$value));
            (attributes, child_nodes)
        }
    };
    ($key:ident = $value:literal $($tail:tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            attributes.push(attribute!($key=$value));
            (attributes, child_nodes)
        }
    };
    ($key:ident-$key_tail:ident = $value:expr, $($tail:tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            attributes.push(attribute!($key-$key_tail=$value));
            (attributes, child_nodes)
        }
    };
    ($key:ident-$key_tail:ident = $value:literal $($tail:tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            attributes.push(attribute!($key-$key_tail=$value));
            (attributes, child_nodes)
        }
    };
    ($value:ident $($tail:tt)*) => {
        {
            let (mut attributes, child_nodes) = attributes_nodes!($($tail)*);
            attributes.push(attribute!($value));
            (attributes, child_nodes)
        }
    };
}

/// Create Element Node

#[macro_export]
macro_rules! element {
    ($element: expr, $($tail: tt)*) => {
        {
            let element =  $element;
            match element {

                Node::Element {
                    node_name,
                    attributes,
                    child_nodes,
                } => {
                    let (mut new_attributes, mut new_child_nodes) = attributes_nodes!($($tail)*);
                    for attribute in attributes {
                        new_attributes.push(attribute);
                    }
                    for node in child_nodes {
                        new_child_nodes.push(node);
                    }
                    Node::Element {
                        node_name,
                        attributes: new_attributes,
                        child_nodes: new_child_nodes,
                    }
                },
                _ => element
            }

        }
    };
    ($node_name: ident $($tail: tt)*) => {
        {
            let (attributes, child_nodes) = attributes_nodes!($($tail)*);
            Node::Element {
                node_name: stringify!($node_name).to_string(),
                attributes,
                child_nodes
            }
        }
    };
    (!DOCTYPE html) => {
        {
            Node::Element {
                node_name: "!DOCTYPE".to_string(),
                attributes: attributes!(html),
                child_nodes: nodes!()
            }
        }
    };

}

/// Copy from https://www.itranslater.com/qa/details/2582473408575439872
fn word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str()
    }
}
#[macro_export]
/// Define Custom Element Node
macro_rules! define {

    ($name: ident $(-$tail: ident)+) => {
        {
            let tag_name = stringify!($name);
            let mut class_name = word(tag_name);
            let mut tag_name = tag_name.to_string();

            $(
                let tail = stringify!($tail);
                let upper_tail = word(tail);
                tag_name = format!("{}-{}", tag_name, tail);
                class_name = format!("{}{}", class_name, upper_tail);

            )+

            element!(script => 
                text!(
                    format!("class PopUpInfo extends HTMLElement {{ constructor() {{ super(); }} }}\ncustomElements.define('{}', {});", tag_name, class_name)
                ),
            )
        }
    };
}

/// Create Text node

#[macro_export]
macro_rules! text {
    ($value: expr) => {
        Node::Text {
            node_value: $value.to_string(),
        }
    };
}

/// Create Comment node

#[macro_export]
macro_rules! comment {
    ($value: expr) => {
        Node::Comment {
            node_value: $value.to_string(),
        }
    };
}

/// Creat default doctype and html

impl Node {
    pub fn default_doctype() -> Node {
        element!(!DOCTYPE html)
    }

    pub fn default_html(title: &str) -> Node {
        element!(html charset="UTF-8" =>
            element!(head =>
                element!(meta charset="UTF-8"),
                element!(meta http-equiv="X-UA-Compatible" content="IE=edge"),
                element!(meta name="viewport" content="width=device-width, initial-scale=1.0"),
                element!(title => text!(title),),
                define!(my-custom),
            ),
            element!(body => text!(""),),
        )
    }
}

/// Display node

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Comment { node_value } => {
                write!(
                    f,
                    "{start}{node_value}{end}",
                    start = "<!--",
                    node_value = node_value,
                    end = "-->"
                )
            }
            Node::Text { node_value } => {
                write!(f, "{}", node_value)
            }
            Node::Element {
                node_name,
                attributes,
                child_nodes,
            } => {
                write!(f, "<{}", node_name)?;
                for attribute in attributes.iter().rev() {
                    write!(f, " {}", attribute)?;
                }

                write!(f, ">")?;
                if child_nodes.len() == 0 {
                    write!(f, "")
                } else {
                    for node in child_nodes.iter().rev() {
                        write!(f, "{}", node)?;
                    }
                    write!(f, "</{}>", node_name)
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_node() {
    let text = Node::Text {
        node_value: "hello world".to_string(),
    };
    let comment = Node::Comment {
        node_value: "comment".to_string(),
    };

    let doctype = Node::Element {
        node_name: "!DOCTYPE".to_string(),
        attributes: attributes!(html),
        child_nodes: Vec::<Node>::with_capacity(0),
    };
    let a = Node::Element {
        node_name: "a".to_string(),
        attributes: attributes!(hidden style="display:None"),
        child_nodes: Vec::<Node>::with_capacity(0),
    };

    let macro_text = text!("hello world");
    let macro_comment = comment!("comment");

    let macro_doctype = element!(!DOCTYPE html);
    let macro_a = element!(a hidden style="display:None");

    assert_eq!(text, macro_text);
    assert_eq!(comment, macro_comment);
    assert_eq!(doctype, macro_doctype);

    assert_eq!(a, macro_a);
    assert_eq!(
        "<a hidden style=\"display:None\">".to_string(),
        macro_a.to_string()
    );

    let nodes = nodes!(head body);
    assert_eq!(vec![element!(body), element!(head),], nodes);
}

#[cfg(test)]
#[test]
fn test_expression() {
    let title = "Topus";
    let html = element!(html charset="UTF-8" =>
        element!(head =>
            element!(meta charset="UTF-8"),
            element!(meta http-equiv="X-UA-Compatible" content="IE=edge"),
            element!(meta name="viewport" content="width=device-width, initial-scale=1.0"),
            element!(title => text!(title),),
        ),
        element!(body => text!(""),),
    );
    assert_eq!("<html charset=\"UTF-8\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Topus</title></head><body></body></html>", html.to_string());
}
