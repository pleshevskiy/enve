# itconfig

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


## Example usage

```rust
#[macro_use] extern crate itconfig;
use dotenv::dotenv;

config! {
    DATABASE_URL: bool,
    HOST: String => "127.0.0.1".to_string(),
}

fn main () {
    dotenv().ok();
    cfg::init();
    assert_eq(cfg::HOST(), String::from("127.0.0.1");
}
```


[documentation]: https://docs.rs/itconfig
