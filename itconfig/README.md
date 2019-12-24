# itconfig

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


## Example usage

```rust
#[macro_use] extern crate itconfig;
// use dotenv::dotenv;

config! {
    DEBUG: bool => true,
    HOST: String => "127.0.0.1".to_string(),
}

fn main () {
    // dotenv().ok();
    cfg::init();
    assert_eq(cfg::HOST(), String::from("127.0.0.1");
}
```


## Roadmap

* [ ] Add namespace for variables
* [ ] Custom env name
* [ ] Add if condition for feature variables


## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.



[documentation]: https://docs.rs/itconfig
