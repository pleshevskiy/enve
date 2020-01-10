# itconfig

Easy build a configs from environment variables and use it in globally.

We recommend you start with the [documentation].


## Example usage

```rust
#[macro_use] extern crate itconfig;
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
        ARTICLE {
            PER_PAGE: u32 => 15,
        }
        
        #[cfg(feature = "companies")]
        COMPANY {
            #[env_name = "INSTITUTIONS_PER_PAGE"]
            PER_PAGE: u32 => 15,
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
    
    cfg::init();
    assert_eq!(cfg::HOST(), String::from("127.0.0.1"));
    assert_eq!(cfg::DATABASE_URL(), String::from("postgres://user:pass@localhost:5432/test"));
    assert_eq!(cfg::APP:ARTICLE:PER_PAGE(), 15);
    assert_eq!(cfg::FEATURE::NEW_MENU(), true);
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
* [ ] Support array type
* [ ] Support hashmap type
* [ ] Support custom env type
* [ ] Common configuration for namespace variables


## License

[MIT] © [Ice Temple](https://github.com/icetemple)


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.



[documentation]: https://docs.rs/itconfig
