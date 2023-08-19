use std::fmt::Display;

pub trait OptionExt {
    fn unwrap_or_unknown(self) -> String;
}

impl<T: Display> OptionExt for Option<T> {
    fn unwrap_or_unknown(self) -> String {
        self.map(|f| f.to_string()).unwrap_or("unknown".to_string())
    }
}
