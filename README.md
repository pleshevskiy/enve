# itconfig

[![CI](https://github.com/icetemple/itconfig-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/icetemple/itconfig-rs/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Documentation](https://docs.rs/itconfig/badge.svg)](https://docs.rs/itconfig)
[![Crates.io](https://img.shields.io/crates/v/itconfig)](https://crates.io/crates/itconfig)
![Crates.io](https://img.shields.io/crates/l/itconfig)

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].

## Motivation

I began to use rust with web programming experience where environment variables
are widely used and often there are more then 50 of them. First I looked at
already created libraries. But there it's necessary to initialise structure that
needs to be moved to each function where you need variable. It uses little bit
memory, but configuration lifetime is as long as application lifetime. Because
of it I decided to create my own library.

## Installation

The MSRV is 1.39.0

Add `itconfig = { version = "1.0", features = ["macro"] }` as a dependency in
`Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
itconfig = { version = "1.0", features = ["macro"] }
```

## Basic usage

```rust
use itconfig::config;
use std::env;
//use dotenv::dotenv;

config! {
    DEBUG: bool => false,

    #[env_name = "APP_HOST"]
    HOST: String => "127.0.0.1",

    database {
        URL < (
            "postgres://",
            POSTGRES_USERNAME => "user",
            ":",
            POSTGRES_PASSWORD => "pass",
            "@",
            POSTGRES_HOST => "localhost:5432",
            "/",
            POSTGRES_DB => "test",
        ),

        pool {
            MAX_SIZE: usize => 15,
        },
    },

    sentry {
        DSN: Option<&'static str>,
    },

    feature {
        static CORS: bool => false,

        static GRAPHQL_PLAYGROUND: bool => false,
    },
}

fn main () {
    // dotenv().expect("dotenv setup to be successful");
    // or
    env::set_var("FEATURE_CORS", "true");

    config::init();
    assert_eq!(config::HOST(), String::from("127.0.0.1"));
    assert_eq!(config::database::URL(), String::from("postgres://user:pass@localhost:5432/test"));
    assert_eq!(config::database::pool::MAX_SIZE(), 15);
    assert_eq!(config::sentry::DSN(), None);
    assert_eq!(config::feature::CORS(), true);
}
```

Macro is an optional feature, disabled by default. You can use this library
without macro

```rust
use itconfig::*;
use std::env;
// use dotenv::dotenv;

fn main() {
    // dotenv().expect("dotenv setup to be successful");
    // or
    env::set_var("DATABASE_URL", "postgres://127.0.0.1:5432/test");

    let database_url = get_env::<String>("DATABASE_URL").unwrap();
    let new_profile: bool = get_env_or_default("FEATURE_NEW_PROFILE", false);
    let articles_per_page: u32 = get_env_or_set_default("ARTICLES_PER_PAGE", 10);
}
```

## Running tests

```bash
cargo test --all-features
```

## Available features

- **default** - ["primitives"]
- **macro** - Activates `config!` macros for easy configure web application.
- **primitives** - Group for features: `numbers` and `bool`.
- **numbers** - Group for features: `int`, `uint` and `float`.
- **int** - Group for features: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`.
- **uint** - Group for features: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
- **float** - Group for features: `f32` and `f64`
- **i8** - impl EnvString for `i8` type
- **i16** - impl EnvString for `i16` type
- **i32** - impl EnvString for `i32` type
- **i64** - impl EnvString for `i64` type
- **i128** - impl EnvString for `i128` type
- **isize** - impl EnvString for `isize` type
- **u8** - impl EnvString for `u8` type
- **u16** - impl EnvString for `u16` type
- **u32** - impl EnvString for `u32` type
- **u64** - impl EnvString for `u64` type
- **u128** - impl EnvString for `u128` type
- **usize** - impl EnvString for `usize` type
- **f32** - impl EnvString for `f32` type
- **f64** - impl EnvString for `f64` type
- **bool** - impl EnvString for `bool` type
- **json_array** - Add EnvString impl for vector type (uses optional
  `serde_json` package). ⚠ **_DEPRECATED_**

## License

[MIT] © [Ice Temple](https://github.com/icetemple)

## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator,
maintainer.

[documentation]: https://docs.rs/itconfig
[MIT]: https://github.com/icetemple/itconfig-rs/blob/master/LICENSE
