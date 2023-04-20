# Meanscout Api (RUST EDITION)
> **Warning** 
<br />THIS IS THE NEW BRANCH
<br/> IF YOU WOULD LIKE TO RUN THIS, SWITCH TO THE MAIN BRANCH
<br/> MANY AREAS MENTIONING SPECIFIC LINES IN THIS README ARE ALSO PROBABLY WRONG 

An attempt at making the meanscout api in rust for maybe better safety<br>
(and honestly I trust this version a lot more)

This API is made for the companion webapp we forked from Aidan Linerud of FRC Team 2471, [Meanscout](https://github.com/4198-Programmers/MeanScout_4198). The exact version of Meanscout we use is under the branch called [rust](https://github.com/4198-Programmers/MeanScout_4198/tree/rust). The API is made to intake and process the data coming from Meanscout. It will then put it into the file `data.csv`.

## How to use

### Running
You can just use it by running `cargo run` in the directory the project is located

### Building as a standalone executable
To build it from source you first need to have [rust](https://rust-lang.org) installed, and then cd into the directory and use the command `cargo build --release`<br>
When it is done building, the binary will be located in `/target/release/` as `meanapi`

### **DO NOT USE THE MAKEFILE YET**

## How to set configurations
### IP Address and Ports
To change the ip to run on just change the address variable in [src/main.rs](https://github.com/4198-Programmers/meanscout-api-rust/blob/main/src/main.rs) before building
```rust
// line 103-110
let config = rocket::Config::figment()
    // The address is set to 0.0.0.0 so it sets the ip to whatever the public network ip is
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000))
    // Replace the file paths below with wherever your needed pem files are for the right certifications
    // Or comment it out if you want to live the dangerous life
    .merge(("tls.certs", "/etc/letsencrypt/live/data.team4198.org/fullchain.pem"))
    .merge(("tls.key", "/etc/letsencrypt/live/data.team4198.org/privkey.pem"));
```

### Passwords
Changing the passwords requires changing the password variable on [Line 47](https://github.com/4198-Programmers/meanscout-api-rust/blob/main/src/main.rs#L44) in [src/main.rs](https://github.com/4198-Programmers/meanscout-api-rust/blob/main/src/main.rs). You add or remove strings to the list to add or remove possible passwords. The strings must have the function `to_string()` attributed to it.
```rust
// Line 47
let passwords = ["GenericPassword".to_string(), "OtherPassword".to_string()];
```

## Logging

### ***THIS SYSTEM IS STILL UNDER DEVELOPMENT***

The current logging system is set of macros that are called when something happens. This writes whether it was successful, or if something wrong happened into the file `logs/meanscout.log`. 

## Contributing

If you would like to contribute to the project you can do any of the following.
* Make a fork and commit and merge your changes
* Add anything like bugs or anything you want added to the issues page


## Credits
<a href="https://github.com/4198-Programmers/meanscout-api-rust/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=4198-Programmers/meanscout-api-rust" />
</a>

Special thanks to <a href="https://github.com/jmelancon"> Joseph Melancon</a> for kicking off our teams digital scouting efforts advising us and much more
