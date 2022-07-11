# Tram-Checker
CLI application to get upcoming tram/departures from a given tram stop in Gothenburg, it uses the [Västtrafik API](https://developer.vasttrafik.se/portal/#/) to accomplish this. Written in Rust.

## Screenshot
![alt text](https://github.com/sandmanlukas/Tram-Checker/blob/master/tram_check.png "Screenshot of Tram-Checker v0.1.1gi")



## Installation

### For Mac

```bash
brew tap sandmanlukas/tap
brew install tram_checker

```
A Västtrafik API-token needs to be generated to be able to use the app, a base64 encoded string. 
This is easily done by creating and app at [Västtrafik API](https://developer.vasttrafik.se/portal/#/) and then using the key and secret values running this command:
```bash
echo "\<key\>:\<secret\>" | base64
```

and then adding this value to your ```~/.zshrc``` or ```~/.bashrc``` files as the following:
```bash
export VASTTRAFIK_TOKEN = \<token\>
```

More information about how Västtrafiks OAuth2 authentification works can be found [here](https://developer.vasttrafik.se/portal/#/guides/oauth2).
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
