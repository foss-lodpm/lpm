#![forbid(unsafe_code)]
#![feature(io_error_more, io_error_uncategorized)]

#[macro_export]
macro_rules! ERRL {
  ($format: expr, $($args: tt)+) => {format! (concat! ("[{}:{}] ", $format), $crate::filename (file!()), line!(), $($args)+)};
  ($format: expr) => {format! (concat! ("[{}:{}] ", $format), $crate::filename (file!()), line!())}}

#[non_exhaustive]
#[derive(Debug)]
pub enum RuntimeErrorKind {
    UnsupportedPlatform(Option<String>),
}

impl RuntimeErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            RuntimeErrorKind::UnsupportedPlatform(_) => "UnsupportedPlatform",
        }
    }

    pub fn throw(&self) -> RuntimeError {
        match self {
            Self::UnsupportedPlatform(ref err) => RuntimeError {
                kind: self.as_str().to_string(),
                reason: err
                    .as_ref()
                    .unwrap_or(&String::from(
                        "LodPM can only work on Linux based platforms.",
                    ))
                    .to_owned(),
            },
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub kind: String,
    pub reason: String,
}

pub mod db;
mod io;
pub mod pkg;
