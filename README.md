# rust-learn

## Cargo.toml

[profile.dev]
opt-level = 1

[profile.release]
panic = 'abort'
strip = "debuginfo"
## Strings

`String -> Vec<u8>`
`&str -> &[u8]`

### String Type

`String` -> Create/modify strings


### String Ref Type

`&str` -> Read/analyze strings


`Box<str>` -> Can be used like &str, but cannot be modified any further

`let my_str: String = String::from("Jeremy");`

`let my_box_str: Box<str> = my_str.into_boxed_str();`


`Rc<str>` -> Share immutable string slice without cloning

`Arc<str>` -> Shared and thread safe string slice without cloning
`let name: Arc<str> = "Jeremy".into();`

## Iter vs Into_Iter

`iter()` for read-only operations

`into_iter()` when you need to take ownership and potentially mutate or consume the data

