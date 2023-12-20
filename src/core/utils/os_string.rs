use std::ffi::{OsStr};

pub trait OsStringConv {
    fn to_string(&self) -> String;
}

impl OsStringConv for OsStr {
    fn to_string(&self) -> String {
        self.to_str()
            .map(ToString::to_string)
            .unwrap_or_default()
    }
}
