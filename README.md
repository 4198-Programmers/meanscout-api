# Meanscout Api (RUST EDITION)
An attempt at making the meanscout api in rust for maybe better safety<br>
(and honestly I trust this version a lot more)

## How to Build
To build it from source you first need to have [rust](https://rust-lang.org) installed, and then cd into the directory and use the command `cargo build`<br>
When it is done building, the binary will be located in `/target/debug/` as `meanapi`

## How to set ip to run on
To change the ip to run on just change the address variable in [Rocket.toml](https://github.com/4198-Programmers/meanscout-api-rust/blob/main/Rocket.toml) before building
```toml
[global]
address = "0.0.0.0" # Before
address = "127.0.0.1" # After
port = 80
```