# port-scanner
A port scanner implemented in rust.

## Build

``` bash
make build
```

## Usage

``` bash
./port-scanner --help

Usage:
port-scanner -h host -p port1[-port2]

Example:
1) Scan 22 port of 127.0.0.1
port-scanner -h 127.0.0.1 -p 22
2) Scan the ports 8000-9000 of 127.0.0.1
port-scanner -h 127.0.0.1 -p 8000-9000
```
