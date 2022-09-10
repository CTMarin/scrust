use std::fmt;
use colored::{Colorize};

/// Represents the all states of a port during a port scan
pub enum PortState {
    Open,
    Closed,
    Filter
}

impl PortState {
    pub fn colorize(&self, output: String) -> String {
        match *self {
            PortState::Open => output.green().to_string(),
            PortState::Closed => output.red().to_string(),
            PortState::Filter => output.yellow().to_string(),
        }
    }
}

impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PortState::Open => write!(f, "open"),
            PortState::Closed => write!(f, "closed"),
            PortState::Filter => write!(f, "filter")
        }
    }
}