// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl LogLevel {
    fn to_str(&self) -> String {
        match self {
            LogLevel::Info => String::from("[INFO]"),
            LogLevel::Warning => String::from("[WARNING]"),
            LogLevel::Error => String::from("[ERROR]"),
        }
    }
}

pub fn log(level: LogLevel, message: &str) -> String {
    level.to_str() + ": " + message
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
