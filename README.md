# rust-learn

## Strings

### String Type

String -> Create/modify strings


### String Ref Type

&str -> Read/analyze strings

Box<str> -> Can be used like &str, but cannot be modified any further
//let my_str: String = String::from("Jeremy");
//let my_box_str: Box<str> = my_str.into_boxed_str();

Rc<str> -> Share immutable string slice without cloning
Arc<str> -> Shared and thread safe string slice without cloning



