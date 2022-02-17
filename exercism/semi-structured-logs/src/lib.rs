// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::char::ToUppercase;
use std::fmt;

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Error => write!(f, "ERROR")
        }
    }
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => return info(message),
        LogLevel::Warning => return warn(message),
        LogLevel::Error => return error(message)
    } 
}

pub fn info(message: &str) -> String {
    return format!("[{}]: {}", str::to_uppercase(&LogLevel::Info.to_string()), message);
}
pub fn warn(message: &str) -> String {
    return format!("[{}]: {}", str::to_uppercase(&LogLevel::Warning.to_string()), message);
}
pub fn error(message: &str) -> String {
    return format!("[{}]: {}", str::to_uppercase(&LogLevel::Error.to_string()), message);
}
