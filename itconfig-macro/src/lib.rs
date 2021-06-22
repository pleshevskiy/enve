#![recursion_limit = "256"]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![forbid(non_ascii_idents)]

mod ast;
mod expand;
mod parse;
mod utils;

extern crate proc_macro;
extern crate proc_macro2;

use self::proc_macro::TokenStream;
use ast::RootNamespace;
use quote::ToTokens;
use syn::parse_macro_input;

/// ### _This API requires the following crate features to be activated: `macro`_
///
/// Creates new public mod with function fo get each environment variable of mapping.
///
/// All variables are required and program will panic if some variables haven't value, but you
/// can add default value for specific variable.
///
/// Starts with v0.6.0 if you don't have an optional variable, the variable is set automatically.
///
/// Example usage
/// -------------
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
/// config! {
///     DATABASE_URL: String,
/// }
///
/// # fn main() {
/// #     env::set_var("DATABASE_URL", "postgres://u:p@localhost:5432/db");
/// #     config::init();
/// # }
/// ```
///
/// Config with default value
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
/// config! {
///     DATABASE_URL: String,
///     HOST: String => "127.0.0.1",
/// }
/// # fn main() {
/// #     env::set_var("DATABASE_URL", "postgres://u:p@localhost:5432/db");
/// #     config::init();
/// # }
/// ```
///
/// By default itconfig lib creates module with 'config' name. But you can use simple meta instruction
/// if you want to rename module. In the example below we renamed module to 'configuration'
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
/// config! {
///     #![config(name = "configuration")]
///
///     DEBUG: bool,
/// }
///
/// fn main() {
///     env::set_var("DEBUG", "t");
///
///     configuration::init();
///     assert_eq!(configuration::DEBUG(), true);
/// }
/// ```
///
/// You also unwrap first config module
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
///
/// config! {
///     #![config(unwrap)]
///
///     DEBUG: bool,
/// }
///
/// fn main() {
///     env::set_var("DEBUG", "t");
///
///     init();
///     assert_eq!(DEBUG(), true);
/// }
/// ```
///
///
/// Namespaces
/// ----------
///
/// You can use namespaces for env variables
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
///
/// config! {
///     DEBUG: bool,
///     DATABASE {
///         USERNAME: String,
///         PASSWORD: String,
///         HOST: String,
///     }
/// }
/// fn main() {
///     env::set_var("DEBUG", "t");
///     env::set_var("DATABASE_USERNAME", "user");
///     env::set_var("DATABASE_PASSWORD", "pass");
///     env::set_var("DATABASE_HOST", "localhost");
///
///     config::init();
/// }
/// ```
///
/// Now you can use nested structure in namespaces without limits :)
///
/// ```rust
/// # use itconfig::config;
/// config! {
///     FIRST {
///         SECOND {
///             THIRD {
///                 FOO: bool => true,
///             }
///         }
///     }
/// }
/// # fn main() { config::init () }
/// ```
///
/// Namespaces supports custom meta
///
/// ```rust
/// # use itconfig::config;
/// config! {
///     #[cfg(feature = "first")]
///     FIRST {
///         #[cfg(feature = "second")]
///         SECOND {
///             #[cfg(feature = "third")]
///             THIRD {
///                 FOO: bool => true,
///             }
///         }
///     }
/// }
/// # fn main() { config::init () }
/// ```
///
/// Meta
/// ----
///
/// If you want to read custom env name for variable you can change it manually.
///
/// **A variable in the nameespace will lose environment prefix**
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
///
/// config! {
///     #[env_name = "MY_CUSTOM_NAME"]
///     PER_PAGE: i32,
///
///     APP {
///         #[env_name = "MY_CUSTOM_NAME"]
///         RECIPES_PER_PAGE: i32,
///     }
/// }
///
/// fn main() {
///     env::set_var("MY_CUSTOM_NAME", "95");
///
///     config::init();
///     assert_eq!(config::PER_PAGE(), 95);
///     assert_eq!(config::APP::RECIPES_PER_PAGE(), 95);
/// }
/// ```
///
/// Also you can add custom meta for each variable. For example feature configurations.
///
/// ```rust
/// # use itconfig::config;
/// config! {
///     #[cfg(feature = "postgres")]
///     DATABASE_URL: String,
///
///     #[cfg(not(feature = "postgres"))]
///     DATABASE_URL: String,
/// }
/// # fn main() { }
/// ```
///
/// Concatenate
/// -----------
///
/// Try to concatenate env variable or strings or both to you env variable. It's easy!
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
/// config! {
///     DATABASE_URL < (
///         "postgres://",
///         POSTGRES_USERNAME,
///         ":",
///         POSTGRES_PASSWORD,
///         "@",
///         POSTGRES_HOST => "localhost:5432",
///         "/",
///         POSTGRES_DB => "test",
///     ),
/// }
///
/// fn main() {
///     env::set_var("POSTGRES_USERNAME", "user");
///     env::set_var("POSTGRES_PASSWORD", "pass");
///
///     config::init();
///     assert_eq!(config::DATABASE_URL(), "postgres://user:pass@localhost:5432/test".to_string());
/// }
/// ```
///
/// Concatinated variables can be only strings and support all features like namespaces and meta.
///
/// ```rust
/// # use itconfig::config;
/// config! {
///     CONCATED_NAMESPACE {
///         #[env_name = "DATABASE_URL"]
///         CONCAT_ENVVAR < (
///             "postgres://",
///             NOT_DEFINED_PG_USERNAME => "user",
///             ":",
///             NOT_DEFINED_PG_PASSWORD => "pass",
///             "@",
///             NOT_DEFINED_PG_HOST => "localhost:5432",
///             "/",
///             NOT_DEFINED_PG_DB => "test",
///         ),
///     }
/// }
/// # fn main() { config::init () }
/// ```
///
/// Static
/// ------
///
/// Starting with 0.11 version you can use lazy static for improve speed of variable. This is very
/// useful, if you use variable more than once.
///
/// ```rust
/// # use itconfig::config;
/// # use std::env;
/// config! {
///     static APP_BASE_URL => "/api",
/// }
///
/// fn main () {
///     env::set_var("APP_BASE_URL", "/api/v1");
///
///     config::init();
///     assert_eq!(config::APP_BASE_URL(), "/api/v1");
/// }
/// ```
///
/// You also can use static with concat variables
///
/// ```rust
/// # use itconfig::config;
/// config! {
///     static CONNECTION_STRING < (
///         "postgres://",
///         NOT_DEFINED_PG_USERNAME => "user",
///         ":",
///         NOT_DEFINED_PG_PASSWORD => "pass",
///         "@",
///         NOT_DEFINED_PG_HOST => "localhost:5432",
///         "/",
///         NOT_DEFINED_PG_DB => "test",
///     ),
/// }
///
/// fn main () {
///     config::init();
///     assert_eq!(config::CONNECTION_STRING(), "postgres://user:pass@localhost:5432/test".to_string());
/// }
/// ```
///
///
/// ---
///
/// This module will also contain helper method:
/// --------------------------------------------
///
/// ```rust
/// pub fn init() {}
/// ```
///
/// Run this at the main function for check all required variables without default value.
///
/// Panics
/// ------
///
/// If you miss some required variables your application will panic at startup.
///
/// Examples
/// --------
///
/// ```rust
/// # use itconfig::config;
/// // use dotenv::dotenv;
///
/// config! {
///     DEBUG: bool => true,
///     HOST: String => "127.0.0.1",
/// }
///
/// fn main () {
///     // dotenv().ok();
///     config::init();
///     assert_eq!(config::HOST(), String::from("127.0.0.1"));
/// }
/// ```
///
#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
    let namespace = parse_macro_input!(input as RootNamespace);
    namespace.into_token_stream().into()
}
