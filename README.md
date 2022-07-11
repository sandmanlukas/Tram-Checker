# Tram-Checker
CLI application to get upcoming tram/departures from a given tram stop in Gothenburg, it uses the [Västtrafik API](https://developer.vasttrafik.se/portal/#/) to accomplish this. Written in Rust.

## Screenshot
![alt text](https://github.com/sandmanlukas/Tram-Checker/blob/master/tram_check.png "Screenshot of Tram-Checker v0.1.1")



## Installation

### For Mac

```bash
brew tap sandmanlukas/tap
brew install tram_checker
```

## Usage

```bash
tram_checker 1.0
Lukas Sandman
A simple CLI-app to get information about coming trams and buses in Gothenburg.

USAGE:
    tram_checker [OPTIONS] <STATION>

ARGS:
    <STATION>    The tram station to display info about

OPTIONS:
    -d, --date <DATE>    If specified, displays information on the coming trams/buses on specified
                         date. Date must be in YYYY-MM-DD format. If not specified, uses current
                         date
    -h, --help           Print help information
    -t, --time <TIME>    If specified, displays information on the coming trams/buses 20min from
                         specified time. Time must be in HH:MM format. If not specified, uses the
                         current time
    -V, --version        Print version information
```
