use std::fmt;
use colored::{Colorize, ColoredString};

/// Represents the all states of a port during a port scan
pub enum PortState {
    Open,
    Closed,
    Filter
}

impl PortState {
    pub fn colorize(&self, output: String) -> ColoredString {
        match *self {
            PortState::Open => output.green(),
            PortState::Closed => output.red(),
            PortState::Filter => output.yellow(),
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

/// Returns a formatted string about the port, its state and service running
pub fn port_info(port: u16, state: PortState, service: String) -> ColoredString {
    let port_info = format!("{}\t\t{}\t\t{}", port, state.to_string(), service);
    state.colorize(port_info)
}