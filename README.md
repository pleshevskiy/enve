# itconfig
[![Build Status](https://travis-ci.org/icetemple/itconfig-rs.svg?branch=master)](https://travis-ci.org/icetemple/itconfig-rs)
[![Documentation](https://docs.rs/itconfig/badge.svg)](https://docs.rs/itconfig)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.4.0-orange.svg?longCache=true)](https://crates.io/crates/itconfig)

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


## Example usage

```rust
#[macro_use] extern crate itconfig;
use std::env;
//use dotenv::dotenv;

config! {
    DEBUG: bool => true,
    HOST: String => "127.0.0.1".to_string(),
    
    NAMESPACE {
        #[env_name = "MY_CUSTOM_NAME"]
        FOO: bool,
        
        BAR: i32 => 10,
        
        #[cfg(feature = "feature")]
        #[env_name = "POSTGRES_CONNECTION_STRING"]
        DATABASE_URL: String
    }
}

fn main () {
    // dotenv().ok();
    env::set_var("MY_CUSTOM_NAME", "t");
    
    cfg::init();
    assert_eq(cfg::HOST(), String::from("127.0.0.1"));
    assert_eq(cfg::NAMESPACE::FOO(), true);
}
```

## Running tests

```bash
cargo test
```


## Roadmap

* [x] Add namespace for variables
* [x] Custom env name
* [x] Support feature config and other meta directives
* [ ] Concat env variables to one variable


## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.


[documentation]: https://docs.rs/itconfig
[MIT]: https://github.com/icetemple/itconfig-rs/blob/master/LICENSE
