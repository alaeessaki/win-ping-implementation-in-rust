# win-ping-cli-implementation-in-rust
ping command implementation in windows using rust winping 

## Installation
```
git clone https://github.com/alaeessaki/win-ping-implementation-in-rust
cd ./is_up/
sudo cargo run -- --help
```

## Usage

```
is_up 1.0
Alae Es-saki
Pings to a server's url

USAGE:
    is_up.exe --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --url <url>    the website url to ping
```

```
cargo run -- --url 8.8.8.8   
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\is_up.exe --url 8.8.8.8`
pinging to 8.8.8.8 ...
Response time 43 ms.
Response time 29 ms.
Response time 53 ms.
Response time 30 ms.
```

## Features
- [x] Ping to Ip address.
- [x] Ping to domain `example.com`.
- [x] works on windows.
- [ ] works on Unix (TODO).

