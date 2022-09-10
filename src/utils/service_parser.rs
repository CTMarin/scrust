use regex::{Captures, Regex, RegexBuilder};


#[cfg(target_family = "unix")]
const SERVICES: &str = include_str!("/etc/services");

#[cfg(target_family = "windows")]
const SERVICES: &str = include_str!("C:\\Windows\\System32\\drivers\\etc\\services");


pub fn service_on_port(port: u16) -> String {
    let pattern: RegexBuilder = RegexBuilder::new(format!("{}", String::from(format!("([a-zA-Z-]+)(\\s*{}/tcp)", port))).as_str());
    let regex: Regex = pattern.build().unwrap();
    let captures: Option<Captures> = regex.captures(SERVICES);
    match captures {
        Some(results) => String::from(&results[1]),
        None =>  String::from("unknown")
    }
}