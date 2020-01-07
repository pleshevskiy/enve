# itconfig

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


## Example usage

```rust
#[macro_use] extern crate itconfig;
use std::env;
//use dotenv::dotenv;

config! {
    DEBUG: bool => true,
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
    assert_eq!(cfg::HOST(), String::from("127.0.0.1"));
    assert_eq!(cfg::DATABASE_URL(), String::from("postgres://user:pass@localhost:5432/test"));
    assert_eq!(cfg::NAMESPACE::FOO(), true);
}
```


## Roadmap

* [x] Add namespace for variables
* [x] Custom env name
* [x] Support feature config and other meta directives
* [x] Add default value to env if env is not found
* [x] Concat env variables to one variable


## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.



[documentation]: https://docs.rs/itconfig
