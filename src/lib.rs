#![allow(dead_code, unused_macros)]

//! # Topus

//! 相较`javascript`，`rust`有更加严谨的语法，编译期能发现bug的优点。既然`javascript`能写`html`，是不是用`rust`写`html`体验更好。基于此想法，我写了`topus`。

//! 我的想法是

//! 1. 构建一个简单`struct DOM`，由上至下有`enum Node`和`enum Attribute`。
//! 2. 实现`Display trait`，通过`to_string()`转换成字符串
//! 3. 创建`html`文件，写入字符串

//! ## Attribute

//! 为了减少学习难度，所有字符串类型为`String`，而不是带有生命周期的`&str`。

//! `enum Attribute`有两个attribute变种。

//! 1. `Boolean(String)`，代表布尔属性，如hidden。
//! 2. `Normal { key: String, value: Sting }`，代表普通属性，如`style="display: None"`。

//! ### 创建方式

//! 1. 直接创建

//! ``` rust
//! let hidden = Attribute::Boolean("hidden".to_string());
//! let style = Attribute::Normal {
//!     key: "style".to_string(),
//!     value: "display: None".to_string()
//! };
//! let http_equiv = Attribute::Normal {
//!     key: "http-equiv".to_string(),
//!     value: "X-UA-Compatible".to_string()
//! };
//! ```

//! 2. 宏创建

//! ``` rust
//! let macro_hidden = attribute!(hidden);
//! let macro_style = attribute!(style="display: None");
//! let macro_http_equiv = attribute!(http-equiv="X-UA-Compatible");
//! ```

//! 推荐使用宏创建`Attribute`，方便快捷。

//! ### 断言

//! ``` rust
//! assert_eq!(hidden, macro_hidden);
//! assert_eq!(style, macro_style);
//! assert_eq!(http_equiv, macro_http_equiv);
//! ```

//! ### 创建Vec<Attribute>

//! 使用attributes宏可以很方便的创建Vec<Attribute>

//! ``` rust
//! let attributes = attributes!(html style="display:None");
//! assert_eq!(
//!     vec![ 
//!         Attribute::Normal{
//!             key: "style".to_string(),
//!             value: "display:None".to_string()
//!         },
//!         Attribute::Boolean("html".to_string())],
//!     attrs);
//! ```

//! 细心的应该发现问题了，`html`和`style="display:None"` 属性是逆向加入Vec容器的。

//! ## Node

//! `enum Node`有三个变种。

//! 1. `  Element { node_name: String, attributes: Vec<Attribute>, child_nodes: Vec<Node>}`，代表`element node`。
//! 2. `Text { node_value: String }`，代表`text node`。
//! 3. `Comment { node_value: String }`，代表`comment node`。

//! ### 创建方式

//! 1. 直接创建

//! ``` rust
//! let text = Node::Text { node_value: "hello world".to_string() }
//! let comment = Node::Comment { node_value: "comment".to_string()}

//! let doctype = Node::Element {
//!     node_name: "!DOCTYPE".to_string(),
//!     attributes: attributes!(html),
//!     child_nodes: Vec::<Node>::with_capacity(0)
//! };
//! let a = Node::Element {
//!     node_name: "a".to_string(),
//!     attributes: attributes!(hidden style="display:None"),
//!     child_nodes: Vec::<Node>::with_capacity(0)
//! };
//! ```

//! 2. 宏创建

//! ``` rust
//! let macro_text = text!("hello world");
//! let macro_comment = comment!("comment");

//! let macro_doctype = element!(!DOCTYPE html);
//! let macro_a = element!(a hidden style="display:None");
//! ```

//! ### 断言

//! ``` rust
//! assert_eq!(text, macro_text);
//! assert_eq!(comment, macro_comment);
//! assert_eq!(doctype, macro_doctype);

//! assert_eq!(a, macro_a);
//! assert_eq!("<a hidden style=\"display:None\">".to_string(), macro_a.to_string());
//! ```

//! 细心的又发现了，`macro_a.to_string()` 中的`hidden` 和`style="display: None"` 属性顺序是正向了，因为在实现`Display trait` 过程中，通过`attributes.iter().rev()`逆转了`attributes`的显示顺序。

//! ### 创建Vec<Node>

//! 使用elements宏可以很方便的创建Vec<Node>

//! ``` rust
//! let nodes = nodes!(head body);
//! assert_eq!(
//!     vec![ 
//!         element!(body),
//!         element!(head),],
//!     attrs);
//! ```

//! 同样的，`head`和`body` 节点是逆序的。

//! ## 使用epxression

//! 在`element!` 宏调用中我们也可以传入表达式参数。如

//! ``` rust
//! let html = element!(html charset="UTF-8" =>
//! 	element!(head =>
//!         element!(meta charset="UTF-8"),
//!         element!(meta http-equiv="X-UA-Compatible" content="IE=edge"),
//!         element!(meta name="viewport" content="width=device-width, initial-scale=1.0"),
//!         element!(title => text!(title),),
//!     ),
//!     element!(body => text!(""),),
//! );
//! ```

//! `Attribute`和`Node`表达式后面要跟着`,`，`Vec<Attribute>`和`Vec<Node>`表达式后面要跟着`;`。

//! ## 生成html文件

//! 通过`build!`宏，生成html文件。

//! ``` rust
//! build!(html => "index.html");
//! ```

use std::fmt;
// Attribute has two varient.
// - Boolean: html, hidden
// - Normal: style="display: None"
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Attribute {
    Boolean(String),
    Normal { key: String, value: String },
}

// Display Attribute

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Attribute::Boolean(value) => {
                write!(f, "{}", value)
            }
            Attribute::Normal { key, value } => {
                write!(f, "{key}=\"{value}\"", key = key, value = value)
            }
        }
    }
}

// Create Attribute
#[macro_export]
macro_rules! attribute {
    ($value:ident) => {
        Attribute::Boolean(stringify!($value).to_string())
    };
    ($key:ident = $value:literal) => {
        Attribute::Normal {
            key: stringify!($key).to_string(),
            value: $value.to_string(),
        }
    };
    ($key:ident = $value:expr) => {
        Attribute::Normal {
            key: stringify!($key).to_string(),
            value: $value.to_string(),
        }
    };
    ($key:ident-$key_tail:ident = $value:literal) => {
        Attribute::Normal {
            key: format!("{}-{}", stringify!($key), stringify!($key_tail)),
            value: $value.to_string(),
        }
    };
    ($key:ident-$key_tail:ident = $value:expr) => {
        Attribute::Normal {
            key: format!("{}-{}", stringify!($key), stringify!($key_tail)),
            value: $value.to_string(),
        }
    };
}

// Create Vec<Attribute>

macro_rules! attributes {
    () => {
        Vec::<Attribute>::new()
    };
    ($attribute: expr, $($tail: tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            attributes.push($attribute);
            attributes
        }
    };
    ($attributes: expr; $($tail: tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            for attribute in $attributes {
                attributes.push($attribute);
            }
            attributes
        }
    };
    ($key:ident = $value:expr, $($tail:tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            attributes.push(attribute!($key=$value));
            attributes
        }
    };
    ($key:ident = $value:literal $($tail:tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            attributes.push(attribute!($key=$value));
            attributes
        }
    };
    ($key:ident-$key_tail:ident = $value:literal $($tail:tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            attributes.push(attribute!($key-$key_tail=$value));
            attributes
        }
    };
    ($key:ident-$key_tail:ident = $value:expr, $($tail:tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            attributes.push(attribute!($key-$key_tail=$value));
            attributes
        }
    };
    ($value:ident $($tail:tt)*) => {
        {
            let mut attributes = attributes!($($tail)*);
            attributes.push(attribute!($value));
            attributes
        }
    };
}

// DOM node has three varient.
// - Element node: node_name attributes child_nodes
// - Text node: node_value is content
// - Comment node: node_value is comment

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

// Creat default doctype and html

impl Node {
    fn default_doctype() -> Node {
        element!(!DOCTYPE html)
    }

    fn default_html(title: &str) -> Node {
        element!(html charset="UTF-8" =>
            element!(head =>
                element!(meta charset="UTF-8"),
                element!(meta http-equiv="X-UA-Compatible" content="IE=edge"),
                element!(meta name="viewport" content="width=device-width, initial-scale=1.0"),
                element!(title => text!(title),),
            ),
            element!(body => text!(""),),
        )
    }
}

// Display node

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

// Create Text node

#[macro_export]
macro_rules! text {
    ($value: expr) => {
        Node::Text {
            node_value: $value.to_string(),
        }
    };
}

// Create Comment node

#[macro_export]
macro_rules! comment {
    ($value: expr) => {
        Node::Comment {
            node_value: $value.to_string(),
        }
    };
}

// Create Element

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

// Parse attributes and elements

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

// Create Vec<Node>

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

// A simplified DOM
pub struct DOM {
    doctype: Node,
    html: Node,
}

impl DOM {
    fn from_title(title: &str) -> Self {
        let doctype = Node::default_doctype();

        let html = Node::default_html(title);

        DOM { doctype, html }
    }
}

impl Default for DOM {
    fn default() -> Self {
        DOM::from_title("Document")
    }
}

impl fmt::Display for DOM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DOM { doctype, html } = self;
        write!(f, "{}{}", doctype, html)
    }
}
// Build html file

#[macro_export]
macro_rules! build {
    ($html: expr => $path: expr) => {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;
        let path = Path::new($path);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all($html.to_string().as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why)
            }
            Ok(_) => println!("successfully wrote to {}", display),
        }
    };
}

#[cfg(test)]
#[test]
fn it_works() {
    let doc = Node::Element {
        node_name: "!DOCTYPE".to_string(),
        attributes: attributes!(html),
        child_nodes: nodes!(),
    };
    assert_eq!("<!DOCTYPE html>".to_string(), doc.to_string());

    let text = text!("hello world");
    assert_eq!("hello world".to_string(), text.to_string());

    let comment = comment!("comment");
    assert_eq!("<!--comment-->".to_string(), comment.to_string());

    let attr = attribute!(html);
    assert_eq!(Attribute::Boolean("html".to_string()), attr);

    let attr = attribute!(style = "display:None");
    assert_eq!(
        Attribute::Normal {
            key: "style".to_string(),
            value: "display:None".to_string()
        },
        attr
    );

    let attrs = attributes!(html style="display:None");
    assert_eq!(
        vec![
            Attribute::Normal {
                key: "style".to_string(),
                value: "display:None".to_string()
            },
            Attribute::Boolean("html".to_string())
        ],
        attrs
    );

    let attrs = attributes!(html html);
    assert_eq!(
        vec![
            Attribute::Boolean("html".to_string()),
            Attribute::Boolean("html".to_string())
        ],
        attrs
    );

    let attrs = attributes!(style="display:None" style="display:None");
    assert_eq!(
        vec![
            Attribute::Normal {
                key: "style".to_string(),
                value: "display:None".to_string()
            },
            Attribute::Normal {
                key: "style".to_string(),
                value: "display:None".to_string()
            }
        ],
        attrs
    );

    let a_element = element!(a);
    assert_eq!("<a>".to_string(), a_element.to_string());
    let a_element = element!(a hidden style="display:None");
    assert_eq!(
        "<a hidden style=\"display:None\">".to_string(),
        a_element.to_string()
    );

    let dom = DOM::default();
    assert_eq!("<!DOCTYPE html><html charset=\"UTF-8\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Document</title></head><body></body></html>".to_string(), dom.to_string());
}
