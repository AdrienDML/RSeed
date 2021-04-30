use std::fmt::{Debug};
use crate::color::*;
use crate::time;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogLevel {
    TRACE,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

impl Into<usize> for LogLevel {
    fn into(self) -> usize {
        match self {
            Self::TRACE => 0,
            Self::INFO => 1,
            Self::WARN => 2,
            Self::ERROR => 3,
            Self::FATAL => 3,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TRACE => write!(f, "Trace"),
            Self::INFO => write!(f, "Info"),
            Self::WARN => write!(f, "Warn"),
            Self::ERROR => write!(f, "Error"),
            Self::FATAL => write!(f, "Fatal"),
        }
    }
}

pub struct Logger {
    name: String,
    colorpicker: Colors,
}

impl Logger {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            colorpicker: Colors::new(),
        }
    }

    pub fn log<T: Debug>(&self, level : LogLevel, name : &String, content : &T) {
        println!(
            "\x1B[{}m[{}] {}:{}> {:?} \x1B[{}m",
            self.colorpicker.get_color(level.into()),
            time::get_time().unwrap(),
            name,
            level,
            content,
            self.colorpicker.get_color(4)
        );
    }

    pub fn trace<T: Debug>(&self, content: &T) {
        self.log(
            LogLevel::TRACE,
            &self.name,
            content,
        );
    }

    pub fn info<T: Debug>(&self, content: &T) {
        self.log(
            LogLevel::INFO,
            &self.name,
            content,
        );
    }
    pub fn warn<T: Debug>(&self, content: &T) {
        self.log(
            LogLevel::WARN,
            &self.name,
            content,
        );
    }
    pub fn error<T: Debug>(&self, content: &T) {
        self.log(
            LogLevel::ERROR,
            &self.name,
            content,
        );
    }
    pub fn fatal<T: Debug>(&self, content: &T) -> ! {
        self.log(
            LogLevel::FATAL,
            &self.name,
            content,
        );
        panic!()
    }
}

