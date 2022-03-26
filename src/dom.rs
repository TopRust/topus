use std::fmt;
use super::node::Node;

/// A simplified DOM
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
/// Build html file

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