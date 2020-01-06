//! # itconfig
//!
//! Simple configuration with macro for rust application.
//!
//!
//! ## Example usage
//!
//! ```rust
//! #[macro_use] extern crate itconfig;
//! use std::env;
//! // use dotenv::dotenv;
//!
//! config! {
//!     DEBUG: bool => true,
//!     HOST: String => "127.0.0.1".to_string(),
//!
//!     NAMESPACE {
//!         #[env_name = "MY_CUSTOM_NAME"]
//!         FOO: bool,
//!
//!         BAR: i32 => 10,
//!     }
//! }
//!
//! fn main () {
//!     // dotenv().ok();
//!     env::set_var("MY_CUSTOM_NAME", "t");
//!
//!     cfg::init();
//!     assert_eq!(cfg::HOST(), String::from("127.0.0.1"));
//!     assert_eq!(cfg::NAMESPACE::FOO(), true);
//! }
//! ```

#[doc(hidden)]
macro_rules! __impl_from_for_numbers {
    ($($ty:ty),+) => {
        $(
            impl From<EnvValue> for $ty {
                fn from(env: EnvValue) -> Self {
                    env.0.parse::<Self>().unwrap()
                }
            }
        )*
    }
}


#[derive(Debug)]
#[doc(hidden)]
pub struct EnvValue(String);

__impl_from_for_numbers![
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
];

impl From<EnvValue> for bool {
    fn from(env: EnvValue) -> Self {
        match env.0.to_lowercase().as_str() {
            "true" | "1" | "t" | "on" => true,
            _ => false,
        }
    }
}

impl From<String> for EnvValue {
    fn from(val: String) -> Self {
        Self(val)
    }
}

impl From<EnvValue> for String {
    fn from(env: EnvValue) -> Self {
        env.0
    }
}


/// Creates new public mod with function fo get each environment variable of mapping.
///
/// All variables are required and program will panic if some variables haven't value, but you
/// can add default value for specific variable.
///
/// Example usage
/// -------------
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// # use std::env;
/// # env::set_var("DATABASE_URL", "postgres://u:p@localhost:5432/db");
/// config! {
///     DATABASE_URL: String,
/// }
/// # cfg::init()
/// ```
///
/// Config with default value
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// # use std::env;
/// # env::set_var("DATABASE_URL", "postgres://u:p@localhost:5432/db");
/// config! {
///     DATABASE_URL: String,
///     HOST: String => "127.0.0.1".to_string(),
/// }
/// # cfg::init()
/// ```
///
/// By default itconfig lib creates module with 'cfg' name. But you can use simple meta instruction
/// if you want to rename module. In the example below we renamed module to 'configuration'
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// # use std::env;
/// env::set_var("DEBUG", "t");
///
/// config! {
///     #![mod_name = configuration]
///
///     DEBUG: bool,
/// }
///
/// configuration::init();
/// assert_eq!(configuration::DEBUG(), true);
/// ```
///
/// You can use namespaces for env variables
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// # use std::env;
/// env::set_var("DEBUG", "t");
/// env::set_var("DATABASE_USERNAME", "user");
/// env::set_var("DATABASE_PASSWORD", "pass");
/// env::set_var("DATABASE_HOST", "localhost");
///
/// config! {
///     DEBUG: bool,
///     DATABASE {
///         USERNAME: String,
///         PASSWORD: String,
///         HOST: String,
///     }
/// }
/// # cfg::init()
/// ```
///
/// If you want to read custom env name for variable you can change it manually.
///
/// **A variable in the nameespace will lose environment prefix**
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// # use std::env;
/// env::set_var("MY_CUSTOM_NAME", "95");
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
/// cfg::init();
/// assert_eq!(cfg::PER_PAGE(), 95);
/// assert_eq!(cfg::APP::RECIPES_PER_PAGE(), 95);
/// ```
///
/// Also you can add custom meta for each variable. For example feature configurations.
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// config! {
///     #[cfg(feature = "postgres")]
///     DATABASE_URL: String,
///
///     #[cfg(not(feature = "postgres"))]
///     DATABASE_URL: String,
/// }
/// # fn main() {}
/// ```
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
/// #[macro_use] extern crate itconfig;
/// // use dotenv::dotenv;
///
/// config! {
///     DEBUG: bool => true,
///     HOST: String => "127.0.0.1".to_string(),
/// }
///
/// fn main () {
///     // dotenv().ok();
///     cfg::init();
///     assert_eq!(cfg::HOST(), String::from("127.0.0.1"));
/// }
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! config {
    ($($tokens:tt)*) => {
        __itconfig_parse_module! {
            tokens = [$($tokens)*],
            name = cfg,
        }
    }
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_invalid_syntax {
    () => {
        compile_error!(
            "Invalid `config!` syntax. Please see the `config!` macro docs for more info.\
            `https://docs.rs/itconfig/latest/itconfig/macro.config.html`"
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_parse_module {
    // Find module name
    (
        tokens = [
            #![mod_name = $mod_name:ident]
            $($rest:tt)*
        ],
        name = $ignore:tt,
    ) => {
        __itconfig_parse_module! {
            tokens = [$($rest)*],
            name = $mod_name,
        }
    };

    // Done parsing module
    (
        tokens = $tokens:tt,
        name = $name:tt,
    ) => {
        __itconfig_parse_variables! {
            tokens = $tokens,
            variables = [],
            namespaces = [],
            module = {
                env_prefix = "",
                name = $name,
            },
        }
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_parse_variables {
    // Find namespace
    (
        tokens = [
            $ns_name:ident { $($ns_tokens:tt)* }
            $($rest:tt)*
        ],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            tokens = [$($ns_tokens)*],
            variables = [],
            module = {
                env_prefix = concat!(stringify!($ns_name), "_"),
                name = $ns_name,
            },
            callback = {
                tokens = [$($rest)*],
                $($args)*
            },
        }
    };

    // Find variable
    (
        tokens = [
            $(#$meta:tt)*
            $name:ident : $ty:ty$( => $default:expr)?,
            $($rest:tt)*
        ],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = [$(#$meta)*],
                meta = [],
                name = $name,
                ty = $ty,
                $(default = $default,)?
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // Find meta with custom env name
    (
        current_variable = {
            unparsed_meta = [
                #[env_name = $env_name:expr]
                $($rest:tt)*
            ],
            meta = $meta:tt,
            name = $name:ident,
            $($current_variable:tt)*
        },
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = [$($rest)*],
                meta = $meta,
                name = $name,
                env_name = $env_name,
                $($current_variable)*
            },
            $($args)*
        }
    };

    // Find stranger meta
    (
        current_variable = {
            unparsed_meta = [
                #$stranger_meta:tt
                $($rest:tt)*
            ],
            meta = [$(#$meta:tt,)*],
            name = $name:ident,
            $($current_variable:tt)*
        },
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = [$($rest)*],
                meta = [$(#$meta,)* #$stranger_meta,],
                name = $name,
                $($current_variable)*
            },
            $($args)*
        }
    };

    // Done parsing variable
    (
        current_variable = {
            unparsed_meta = [],
            $($current_variable:tt)*
        },
        tokens = $tokens:tt,
        variables = [$($variables:tt,)*],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            tokens = $tokens,
            variables = [$($variables,)* { $($current_variable)* },],
            $($args)*
        }
    };

    // Done parsing all variables of namespace
    (
        tokens = [],
        variables = $ns_variables:tt,
        module = {
            $($current_namespace:tt)*
        },
        callback = {
            tokens = $tokens:tt,
            variables = $variables:tt,
            namespaces = [$($namespaces:tt,)*],
            $($args:tt)*
        },
    ) => {
        __itconfig_parse_variables! {
            tokens = $tokens,
            variables = $variables,
            namespaces = [
                $($namespaces,)*
                {
                    variables = $ns_variables,
                    $($current_namespace)*
                },
            ],
            $($args)*
        }
    };

    // Done parsing all variables
    (
        tokens = [],
        $($args:tt)*
    ) => {
        __itconfig_impl!($($args)*);
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_impl {
    (
        variables = [$({
            meta = $var_meta:tt,
            name = $var_name:ident,
            $($variable:tt)*
        },)*],
        namespaces = [$({
            variables = [$({
                meta = $ns_var_meta:tt,
                name = $ns_var_name:ident,
                $($ns_variables:tt)*
            },)*],
            env_prefix = $ns_env_prefix:expr,
            name = $ns_name:ident,
        },)*],
        module = {
            env_prefix = $env_prefix:expr,
            name = $mod_name:ident,
        },
    ) => {
        pub mod $mod_name {
            #![allow(non_snake_case)]
            $(
                pub mod $ns_name {
                    $(__itconfig_variable! {
                        meta = $ns_var_meta,
                        name = $ns_var_name,
                        env_prefix = $ns_env_prefix,
                        $($ns_variables)*
                    })*
                }
            )*

            pub fn init() {
                $($var_name();)*

                $($($ns_name::$ns_var_name();)*)*
            }

            $(__itconfig_variable! {
                meta = $var_meta,
                name = $var_name,
                env_prefix = $env_prefix,
                $($variable)*
            })*
        }
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_variable {
    // Set default env name
    (
        meta = $meta:tt,
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        ty = $ty:ty,
        $($args:tt)*
    ) => {
        __itconfig_variable! {
            meta = $meta,
            name = $name,
            env_prefix = $env_prefix,
            env_name = concat!($env_prefix, stringify!($name)).to_uppercase(),
            ty = $ty,
            $($args)*
        }
    };

    // Add method
    (
        meta = [$(#$meta:tt,)*],
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        env_name = $env_name:expr,
        ty = $ty:ty,
        $(default = $default:expr,)?
    ) => {
        $(#$meta)*
        pub fn $name() -> $ty {
            env_or!($env_name$(, $default)?)
        }
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}

/// This macro returns environment variable by name and converts variable to desired type
/// or returns default value.
///
/// Panics
/// ------
/// If you don't pass default value, macro will panic
///
/// Examples
/// --------
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// let url: String = env_or!("DATABASE_URL", "127.0.0.1".to_string());
/// assert_eq!(url, "127.0.0.1".to_string());
/// ```
#[macro_export(local_inner_macro)]
macro_rules! env_or {
    ($env_name:expr) => {
        env_or!($env_name, format!(r#"Cannot read "{}" environment variable"#, $env_name), panic);
    };

    ($env_name:expr, $default:expr) => {
        env_or!($env_name, $default, default);
    };

    ($env_name:expr, $default:expr, $token:tt) => {{
        use std::env;
        use itconfig::EnvValue;
        env::var($env_name)
            .map(|val| EnvValue::from(val).into())
            .unwrap_or_else(|_| env_or!(@$token $env_name, $default))
    }};

    (@default $env_name:expr, $default:expr) => {{
        env::set_var($env_name, $default.to_string());
        $default
    }};

    (@panic $env_name:expr, $default:expr) => {
        panic!($default);
    };
}

