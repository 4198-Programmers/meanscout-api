# Meanscout Api (RUST EDITION)

An attempt at making the meanscout api in rust for maybe better safety. Can't be *too* easy to make.<br>
(and honestly I trust this version a lot more)

This network API backend is made for the companion webapp we forked from Aidan Linerud of FRC Team 2471, [Meanscout](https://github.com/4198-Programmers/MeanScout_4198). The exact version of Meanscout we use is under the main branch located [here](https://github.com/4198-Programmers/MeanScout_4198/tree/main). The API is made to intake and process the data coming from Meanscout. It will then put it into the file `data.csv`.

## How to use
In the future I plan on putting it on crates.io so you can just run `cargo install meanscout` or something along those lines

### Running
You can just use it by running `cargo run --release` in the directory the project is located

### Building as a standalone executable
To build it from source you first need to have [rust](https://rust-lang.org) installed, and use the command `cargo build --release` while in the project directory<br>
When it is done building, the binary will be located in `/target/release/` as the executable `meanapi`

## How to set configurations
All of the configuration that you should need is located in [Config.toml](https://github.com/4198-Programmers/meanscout-api-rust/blob/axum-rewrite/Config.toml). 
### IP Address and Ports
To change the ip or ports to run on just change the variable in [Config.toml](https://github.com/4198-Programmers/meanscout-api-rust/blob/axum-rewrite/Config.toml) as shown
```toml
ip_address = [0,0,0,0] # Each digit is each octet in the ip address
port = 8000
```

### Passwords
Changing the passwords requires changing the password variable in [settings.toml](https://github.com/4198-Programmers/meanscout-api-rust/blob/axum-rewrite/Config.toml). You add or remove strings to the list to add or remove possible passwords.
```toml
passwords = ["ChangeMe!", "AnotherPassword!"]
```

## Logging

The current logging system is set of macros that are called when something happens. This writes whether it was successful, or if something wrong happened into the file `logs/meanscout.log`, or a different location configured in Config.toml.

## Contributing

If you would like to contribute to the project you can do any of the following.
* Make a fork and commit and merge your changes
* Add anything like bugs or things you'd like added to the issues page


## Credits
<a href="https://github.com/4198-Programmers/meanscout-api-rust/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=4198-Programmers/meanscout-api-rust" />
</a>

Special thanks to <a href="https://github.com/jmelancon"> Joseph Melancon</a> for kicking off our teams digital scouting efforts, advising us, and much more
