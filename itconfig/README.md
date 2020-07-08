# itconfig

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


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
use std::itconfig;
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
    // dotenv().expect("dotenv setup to be successful");
    // or
    env::set_var("FEATURE_NEW_MENU", "t");
    
    config::init();
    assert_eq!(config::HOST(), String::from("127.0.0.1"));
    assert_eq!(config::DATABASE_URL(), String::from("postgres://user:pass@localhost:5432/test"));
    assert_eq!(config::APP:ARTICLE:PER_PAGE(), 15);
    assert_eq!(config::FEATURE::NEW_MENU(), true);
}
```


Macro is an optional feature, disabled by default. You can use this library without macro

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

* **default** - ["primitives"]
* **macro** - Activates `config!` macros for easy configure web application.
* **array** - Add EnvString impl for vector type (uses optional `serde_json` package).
* **primitives** - Group for features: `numbers` and `bool`.
* **numbers** - Group for features: `int`, `uint` and `float`.
* **int** - Group for features: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`.
* **uint** - Group for features: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
* **float** - Group for features: `f32` and `f64`
* **i8** - impl EnvString for `i8` type
* **i16** - impl EnvString for `i16` type
* **i32** - impl EnvString for `i32` type
* **i64** - impl EnvString for `i64` type
* **i128** - impl EnvString for `i128` type
* **isize** - impl EnvString for `isize` type
* **u8** - impl EnvString for `u8` type
* **u16** - impl EnvString for `u16` type
* **u32** - impl EnvString for `u32` type
* **u64** - impl EnvString for `u64` type
* **u128** - impl EnvString for `u128` type
* **usize** - impl EnvString for `usize` type
* **f32** - impl EnvString for `f32` type
* **f64** - impl EnvString for `f64` type
* **bool** - impl EnvString for `bool` type


## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.



[documentation]: https://docs.rs/itconfig
