# Topus

相较`javascript`，`rust`有更加严谨的语法，编译期能发现bug的优点。既然`javascript`能写`html`，是不是用`rust`写`html`体验更好。基于此想法，我写了`topus`。

我的想法是

1. 构建一个简单`struct DOM`，由上至下有`enum Node`和`enum Attribute`。
2. 实现`Display trait`，通过`to_string()`转换成字符串
3. 创建`html`文件，写入字符串

## Attribute

为了减少学习难度，所有字符串类型为`String`，而不是带有生命周期的`&str`。

`enum Attribute`有两个attribute变种。

1. `Boolean(String)`，代表布尔属性，如hidden。
2. `Normal { key: String, value: Sting }`，代表普通属性，如`style="display: None"`。

### 创建方式

1. 直接创建

``` rust
let hidden = Attribute::Boolean("hidden".to_string());
let style = Attribute::Normal {
    key: "style".to_string(),
    value: "display: None".to_string()
};
let http_equiv = Attribute::Normal {
    key: "http-equiv".to_string(),
    value: "X-UA-Compatible".to_string()
};
```

2. 宏创建

``` rust
let macro_hidden = attribute!(hidden);
let macro_style = attribute!(style="display: None");
let macro_http_equiv = attribute!(http-equiv="X-UA-Compatible");
```

推荐使用宏创建`Attribute`，方便快捷。

### 断言

``` rust
assert_eq!(hidden, macro_hidden);
assert_eq!(style, macro_style);
assert_eq!(http_equiv, macro_http_equiv);
```

### 创建Vec<Attribute>

使用attributes宏可以很方便的创建Vec<Attribute>

``` rust
let attributes = attributes!(html style="display:None");
assert_eq!(
    vec![ 
        Attribute::Normal{
            key: "style".to_string(),
            value: "display:None".to_string()
        },
        Attribute::Boolean("html".to_string())],
    attrs);
```

细心的应该发现问题了，`html`和`style="display:None"` 属性是逆向加入Vec容器的。

## Node

`enum Node`有三个变种。

1. `  Element { node_name: String, attributes: Vec<Attribute>, child_nodes: Vec<Node>}`，代表`element node`。
2. `Text { node_value: String }`，代表`text node`。
3. `Comment { node_value: String }`，代表`comment node`。

### 创建方式

1. 直接创建

``` rust
let text = Node::Text { node_value: "hello world".to_string() }
let comment = Node::Comment { node_value: "comment".to_string()}

let doctype = Node::Element {
    node_name: "!DOCTYPE".to_string(),
    attributes: attributes!(html),
    child_nodes: Vec::<Node>::with_capacity(0)
};
let a = Node::Element {
    node_name: "a".to_string(),
    attributes: attributes!(hidden style="display:None"),
    child_nodes: Vec::<Node>::with_capacity(0)
};
```

2. 宏创建

``` rust
let macro_text = text!("hello world");
let macro_comment = comment!("comment");

let macro_doctype = element!(!DOCTYPE html);
let macro_a = element!(a hidden style="display:None");
```

### 断言

``` rust
assert_eq!(text, macro_text);
assert_eq!(comment, macro_comment);
assert_eq!(doctype, macro_doctype);

assert_eq!(a, macro_a);
assert_eq!("<a hidden style=\"display:None\">".to_string(), macro_a.to_string());
```

细心的又发现了，`macro_a.to_string()` 中的`hidden` 和`style="display: None"` 属性顺序是正向了，因为在实现`Display trait` 过程中，通过`attributes.iter().rev()`逆转了`attributes`的显示顺序。

### 创建Vec<Node>

使用elements宏可以很方便的创建Vec<Node>

``` rust
let nodes = nodes!(head body);
assert_eq!(
    vec![ 
        element!(body),
        element!(head),],
    attrs);
```

同样的，`head`和`body` 节点是逆序的。

## 使用epxression

在`element!` 宏调用中我们也可以传入表达式参数。如

``` rust
let html = element!(html charset="UTF-8" =>
	element!(head =>
        element!(meta charset="UTF-8"),
        element!(meta http-equiv="X-UA-Compatible" content="IE=edge"),
        element!(meta name="viewport" content="width=device-width, initial-scale=1.0"),
        element!(title => text!(title),),
    ),
    element!(body => text!(""),),
);
```

`Attribute`和`Node`表达式后面要跟着`,`，`Vec<Attribute>`和`Vec<Node>`表达式后面要跟着`;`。

## 生成html文件

通过`build!`宏，生成html文件。

``` rust
build!(html => "index.html");
```