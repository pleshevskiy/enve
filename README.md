# itconfig
[![Build Status](https://travis-ci.org/icetemple/itconfig-rs.svg?branch=master)](https://travis-ci.org/icetemple/itconfig-rs)
[![Documentation](https://docs.rs/itconfig/badge.svg)](https://docs.rs/itconfig)
[![Crates.io](https://img.shields.io/badge/crates.io-v1.0.0-blue.svg?longCache=true)](https://crates.io/crates/itconfig) 
[![Join the chat at https://gitter.im/icetemple/itconfig-rs](https://badges.gitter.im/icetemple/itconfig-rs.svg)](https://gitter.im/icetemple/itconfig-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


## Motivation

I began to use rust with web programming experience where environment variables are widely used 
and often there are more then 50 of them. First I looked at already created libraries. 
But there it's necessary to initialise structure that needs to be moved to each function 
where you need variable. It uses little bit memory, but configuration lifetime is as long 
as application lifetime. Because of it I decided to create my own library.


## Installation

These macros require a Rust compiler version 1.31 or newer.

Add `itconfig = { version = "1.0", features = ["macro"] }` as a dependency in `Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
itconfig = { version = "1.0", features = ["macro"] }
```


## Example usage

```rust
use itconfig::config;
use std::env;
//use dotenv::dotenv;

config! {
    DEBUG: bool => false,
    
    #[env_name = "APP_HOST"]
    HOST: String => "127.0.0.1",
    
    DATABASE_URL < (
        "postgres://",
        POSTGRES_USERNAME => "user",
        ":",
        POSTGRES_PASSWORD => "pass",
        "@",
        POSTGRES_HOST => "localhost:5432",
        "/",
        POSTGRES_DB => "test",
    ),
    
    APP {
        static BASE_URL => "/api", // &'static str by default
    
        ARTICLE {
            static PER_PAGE: u32 => 15,
        }
        
        #[cfg(feature = "companies")]
        COMPANY {
            #[env_name = "INSTITUTIONS_PER_PAGE"]
            static PER_PAGE: u32 => 15,
        }
    }
    
    FEATURE {
        NEW_MENU: bool => false,
    
        COMPANY {
            PROFILE: bool => false,
        }
    }
}

fn main () {
    // dotenv().ok();
    env::set_var("FEATURE_NEW_MENU", "t");
    
    config::init();
    assert_eq!(config::HOST(), String::from("127.0.0.1"));
    assert_eq!(config::DATABASE_URL(), String::from("postgres://user:pass@localhost:5432/test"));
    assert_eq!(config::APP:ARTICLE:PER_PAGE(), 15);
    assert_eq!(config::FEATURE::NEW_MENU(), true);
}
```


Macro is an optional feature, enabled by default. You can install itconfig without default
features and use this lib as shown below

```rust
use itconfig::*;
use std::env;
// use dotenv::dotenv;

fn main() {
    env::set_var("DATABASE_URL", "postgres://127.0.0.1:5432/test");

    let database_url = get_env::<String>("DATABASE_URL").unwrap();
    let new_profile: bool = get_env_or_default("FEATURE_NEW_PROFILE", false);
    let articles_per_page: u32 = get_env_or_set_default("ARTICLES_PER_PAGE", 10);
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
* [x] Add default value to env if env is not found
* [x] Concat env variables to one variable
* [x] Add nested namespaces
* [x] Support meta for namespaces
* [x] Support array type
* [x] Rewrite to proc macro
* [ ] Support hashmap type
* [ ] Support custom env type
* [ ] Common configuration for namespace variables


## Available features

* **default** - ["macro", "primitives", "static"]
* **macro** - Activates `config!` macros for easy configure web application.
* **array** - Add EnvString impl for vector type (uses optional `serde_json` package).
* **primitives** - Group for features: `numbers` and `bool`.
* **numbers** - Group for features: `int`, `uint` and `float`.
* **int** - Group for features: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`.
* **uint** - Group for features: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
* **float** - Group for features: `f32` and `f64`
* **i8** - impl EnvString for i8 type
* **i16** - impl EnvString for i16 type
* **i32** - impl EnvString for i32 type
* **i64** - impl EnvString for i64 type
* **i128** - impl EnvString for i128 type
* **isize** - impl EnvString for isize type
* **u8** - impl EnvString for u8 type
* **u16** - impl EnvString for u16 type
* **u32** - impl EnvString for u32 type
* **u64** - impl EnvString for u64 type
* **u128** - impl EnvString for u128 type
* **usize** - impl EnvString for usize type
* **f32** - impl EnvString for f32 type
* **f64** - impl EnvString for f64 type
* **bool** - impl EnvString for bool type

## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.


[documentation]: https://docs.rs/itconfig
[MIT]: https://github.com/icetemple/itconfig-rs/blob/master/LICENSE
