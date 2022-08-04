# Meanscout Api (RUST EDITION)
An attempt at making the meanscout api in rust for maybe better safety<br>
(and honestly I trust this version a lot more)

## How to run
You can just use it by running `cargo run` in the directory the project is located

## How to Build
To build it from source you first need to have [rust](https://rust-lang.org) installed, and then cd into the directory and use the command `cargo build --release`<br>
When it is done building, the binary will be located in `/target/release/` as `meanapi`

## How to set ip to run on
To change the ip to run on just change the address variable in [main.rs](https://github.com/4198-Programmers/meanscout-api-rust/blob/main/src/main.rs) before building
```rust
// line 42-44
let config = rocket::Config::figment()
    .merge(("address", "0.0.0.0")) // Change the ip address here
    .merge(("port", 80)); // Change the port here
```
