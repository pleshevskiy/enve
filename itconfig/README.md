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
    HOST: String => "127.0.0.1".to_string(),
    
    NAMESPACE {
        #[env_name = "MY_CUSTOM_NAME"]
        FOO: bool,
        
        BAR: i32 => 10,
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


## Roadmap

* [x] Add namespace for variables
* [x] Custom env name
* [ ] Add if condition for feature variables


## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.



[documentation]: https://docs.rs/itconfig
