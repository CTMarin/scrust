# Scrust
A simple and fast port scanner written in Rust

## Usage
```scala
USAGE:
    scrust [OPTIONS] <ADDRESS>

ARGS:
    <ADDRESS>

OPTIONS:
    -e, --end-port <END_PORT>         [default: 65535]
    -h, --help                        Print help information
    -i, --initial-port <INIT_PORT>    [default: 1]
    -V, --version                     Print version information
```

## TODO List
- [x] Performance
    - [x] Multithreading
- [ ] Scanning
    - [x] Scan IP Address
    - [ ] Scan Host Name
    - [ ] Multiple Addresses
    - [ ] Filtered Ports
- [ ] Information
    - [ ] Service Running
    - [ ] Vulnerabilities
