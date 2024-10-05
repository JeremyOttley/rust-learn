use std::path::Path;
use std::ffi::OsStr;

fn get_file_extension(path: &str) -> Option<&str> {
    Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
}
