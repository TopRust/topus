use std::fmt;
/// Attribute has two varient.
/// - Boolean: html, hidden
/// - Normal: style="display: None"
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Attribute {
    Boolean(String),
    Normal { key: String, value: String },
}

/// Display Attribute

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

/// Create Attribute
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

/// Create Vec<Attribute>

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

#[cfg(test)]
#[test]
fn test_attribute() {
    let hidden = Attribute::Boolean("hidden".to_string());
    let style = Attribute::Normal {
        key: "style".to_string(),
        value: "display: None".to_string()
    };
    let http_equiv = Attribute::Normal {
        key: "http-equiv".to_string(),
        value: "X-UA-Compatible".to_string()
    };

    let macro_hidden = attribute!(hidden);
    let macro_style = attribute!(style="display: None");
    let macro_http_equiv = attribute!(http-equiv="X-UA-Compatible");

    assert_eq!(hidden, macro_hidden);
    assert_eq!(style, macro_style);
    assert_eq!(http_equiv, macro_http_equiv);

    let attributes = attributes!(html style="display:None");
    assert_eq!(
        vec![ 
            Attribute::Normal{
                key: "style".to_string(),
                value: "display:None".to_string()
            },
            Attribute::Boolean("html".to_string())],
        attributes);
}