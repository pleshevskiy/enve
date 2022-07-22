# enve

[![CI](https://github.com/pleshevskiy/enve/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/pleshevskiy/enve/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Documentation](https://docs.rs/pleshevskiy/badge.svg)](https://docs.rs/enve)
[![Crates.io](https://img.shields.io/crates/v/enve)](https://crates.io/crates/enve)
![Crates.io](https://img.shields.io/crates/l/enve)

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

Add `enve = { version = "1.0", features = ["mod"] }` as a dependency in
`Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
enve = { version = "1.0", features = ["mod"] }
```

## Basic usage

```rust
use std::env;
//use dotenv::dotenv;

enve::mod! {
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
use std::env;
// use dotenv::dotenv;

fn main() {
    // dotenv().expect("dotenv setup to be successful");
    // or
    env::set_var("DATABASE_URL", "postgres://127.0.0.1:5432/test");

    let database_url = enve::get::<String>("DATABASE_URL").unwrap();
    let new_profile: bool = enve::get("FEATURE_NEW_PROFILE").unwrap_or_default();
    let articles_per_page: u32 = enve::get_or_set_default("ARTICLES_PER_PAGE", 10);
}
```

## Running tests

```bash
cargo test --all-features
```

## Available features

- **macro** - Activates `config!` macros for easy configure web application.
- **number** - Group for features: `int`, `uint` and `float`.
- **bool** - impl EnvString for `bool` type `serde_json` package). ⚠
  **_DEPRECATED_**

## License

[MIT] © [pleshevskiy](https://github.com/pleshevskiy)

## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator,
maintainer.

[documentation]: https://docs.rs/enve
[MIT]: https://github.com/icetemple/enve-rs/blob/master/LICENSE
