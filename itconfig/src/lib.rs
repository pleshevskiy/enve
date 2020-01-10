//! # itconfig
//!
//! Simple configuration with macro for rust application.
//!
//!
//! ## Motivation
//!
//! I began to use rust with web programming experience where environment variables are widely used
//! and often there are more then 50 of them. First I looked at already created libraries.
//! But there it's necessary to initialise structure that needs to be moved to each function
//! where you need variable. It uses little bit memory, but configuration lifetime is as long
//! as application lifetime. Because of it I decided to create my own library.
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
//!     HOST: String => "127.0.0.1",
//!
//!     DATABASE_URL < (
//!         "postgres://",
//!         POSTGRES_USERNAME => "user",
//!         ":",
//!         POSTGRES_PASSWORD => "pass",
//!         "@",
//!         POSTGRES_HOST => "localhost:5432",
//!         "/",
//!         POSTGRES_DB => "test",
//!     ),
//!
//!     APP {
//!         ARTICLE {
//!             PER_PAGE: u32 => 15,
//!         }
//!
//!         #[cfg(feature = "companies")]
//!         COMPANY {
//!             #[env_name = "INSTITUTIONS_PER_PAGE"]
//!             PER_PAGE: u32 => 15,
//!         }
//!     }
//!
//!     FEATURE {
//!         NEW_MENU: bool => false,
//!
//!         COMPANY {
//!             PROFILE: bool => false,
//!         }
//!     }
//! }
//!
//! fn main () {
//!     // dotenv().ok();
//!     env::set_var("FEATURE_NEW_MENU", "t");
//!
//!     cfg::init();
//!     assert_eq!(cfg::HOST(), String::from("127.0.0.1"));
//!     assert_eq!(cfg::DATABASE_URL(), String::from("postgres://user:pass@localhost:5432/test"));
//!     assert_eq!(cfg::APP::ARTICLE::PER_PAGE(), 15);
//!     assert_eq!(cfg::FEATURE::NEW_MENU(), true);
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
/// Starts with v0.6.0 if you don't have an optional variable, the variable is set automatically.
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
///     HOST: String => "127.0.0.1",
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
/// Namespaces
/// ----------
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
/// Now you can use nested structure in namespaces without limits :)
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// config! {
///     FIRST {
///         SECOND {
///             THIRD {
///                 FOO: bool => true,
///             }
///         }
///     }
/// }
/// # cfg::init();
/// ```
///
/// Namespaces supports custom meta
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
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
/// # cfg::init();
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
/// Concatenate
/// -----------
///
/// Try to concatenate env variable or strings or both to you env variable. It's easy!
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
/// # use std::env;
/// env::set_var("POSTGRES_USERNAME", "user");
/// env::set_var("POSTGRES_PASSWORD", "pass");
///
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
/// cfg::init();
/// assert_eq!(cfg::DATABASE_URL(), "postgres://user:pass@localhost:5432/test".to_string())
/// ```
///
/// Concatinated variables can be only strings and support all features like namespaces and meta.
///
/// ```rust
/// # #[macro_use] extern crate itconfig;
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
///
/// cfg::init();
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
///     HOST: String => "127.0.0.1",
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
                meta = [],
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
            $(#$meta:tt)*
            $ns_name:ident { $($ns_tokens:tt)* }
            $($rest:tt)*
        ],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            tokens = [$($ns_tokens)*],
            variables = [],
            namespaces = [],
            module = {
                env_prefix = concat!(stringify!($ns_name), "_"),
                name = $ns_name,
                meta = [$(#$meta)*],
            },
            callback = {
                tokens = [$($rest)*],
                $($args)*
            },
        }
    };

    // Find concatenated variable
    (
        tokens = [
            $(#$meta:tt)*
            $name:ident < ($($inner:tt)+),
            $($rest:tt)*
        ],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = [$(#$meta)*],
                meta = [],
                unparsed_concat = [$($inner)+],
                concat = [],
                name = $name,
                ty = String,
            },
            tokens = [$($rest)*],
            $($args)*
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
                unparsed_concat = [],
                concat = [],
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
            unparsed_concat = $unparsed_concat:tt,
            concat = $concat:tt,
            name = $name:ident,
            $($current_variable:tt)*
        },
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = [$($rest)*],
                meta = $meta,
                unparsed_concat = $unparsed_concat,
                concat = $concat,
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
            $($current_variable:tt)*
        },
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = [$($rest)*],
                meta = [$(#$meta,)* #$stranger_meta,],
                $($current_variable)*
            },
            $($args)*
        }
    };

    // Parse concat params
    (
        current_variable = {
            unparsed_meta = $unparsed_meta:tt,
            meta = $meta:tt,
            unparsed_concat = [
                $concat_param:tt$( => $default:expr)?,
                $($rest:tt)*
            ],
            concat = [$($concat:expr,)*],
            $($current_variable:tt)*
        },
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                unparsed_meta = $unparsed_meta,
                meta = $meta,
                unparsed_concat = [$($rest)*],
                concat = [$($concat,)* __itconfig_concat_param!($concat_param$( => $default)?),],
                $($current_variable)*
            },
            $($args)*
        }
    };

    // Done parsing variable
    (
        current_variable = {
            unparsed_meta = [],
            meta = $meta:tt,
            unparsed_concat = [],
            $($current_variable:tt)*
        },
        tokens = $tokens:tt,
        variables = [$($variables:tt,)*],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            tokens = $tokens,
            variables = [$($variables,)* { meta = $meta, $($current_variable)* },],
            $($args)*
        }
    };

    // Done parsing all variables of namespace
    (
        tokens = [],
        variables = $ns_variables:tt,
        namespaces = $ns_namespaces:tt,
        module = $ns_module:tt,
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
                    namespaces = $ns_namespaces,
                    module = $ns_module,
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
        __itconfig_impl_namespace!($($args)*);
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_impl_namespace {
    (
        variables = [$({
            meta = $var_meta:tt,
            concat = $var_concat:tt,
            name = $var_name:ident,
            $($variable:tt)*
        },)*],
        namespaces = [$({
            variables = $ns_variable:tt,
            namespaces = $ns_namespaces:tt,
            module = {
                env_prefix = $ns_env_prefix:expr,
                name = $ns_mod_name:ident,
                meta = [$(#$ns_meta:tt)*],
            },
        },)*],
        module = {
            env_prefix = $env_prefix:expr,
            name = $mod_name:ident,
            meta = [$(#$meta:tt)*],
        },
    ) => {
        $(#$meta)*
        pub mod $mod_name {
            #![allow(non_snake_case)]
            use std::env;
            use itconfig::EnvValue;

            $(__itconfig_impl_namespace! {
                variables = $ns_variable,
                namespaces = $ns_namespaces,
                module = {
                    env_prefix = $ns_env_prefix,
                    name = $ns_mod_name,
                    meta = [$(#$ns_meta)*],
                },
            })*

            pub fn init() {
                $($var_name();)*
                $(
                    $(#$ns_meta)*
                    $ns_mod_name::init();
                )*
            }

            $(__itconfig_variable! {
                meta = $var_meta,
                concat = $var_concat,
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
macro_rules! __itconfig_concat_param {
    // Find env parameter with default value
    ($env_name:ident => $default:expr) => {
        __itconfig_variable_helper!(stringify!($env_name).to_uppercase(), $default, default)
    };

    // Find env parameter without default value
    ($env_name:ident) => {
        env_or!(stringify!($env_name).to_uppercase())
    };

    // Find string parameter
    ($str:expr) => ( $str.to_string() );

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
        concat = $concat:tt,
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        ty = $ty:ty,
        $($args:tt)*
    ) => {
        __itconfig_variable! {
            meta = $meta,
            concat = $concat,
            name = $name,
            env_prefix = $env_prefix,
            env_name = concat!($env_prefix, stringify!($name)).to_uppercase(),
            ty = $ty,
            $($args)*
        }
    };

    // Add method for concatenated variable
    (
        meta = [$(#$meta:tt,)*],
        concat = [$($concat:expr,)+],
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        env_name = $env_name:expr,
        ty = $ty:ty,
        $($args:tt)*
    ) => {
        $(#$meta)*
        pub fn $name() -> $ty {
            let value_parts: Vec<String> = vec!($($concat),+);
            let value = value_parts.join("");
            __itconfig_variable_helper!(@setenv $env_name, value)
        }
    };

    // Add method for env variable
    (
        meta = [$(#$meta:tt,)*],
        concat = [],
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
    // Env without default value
    ($env_name:expr) => {
        __itconfig_variable_helper!($env_name, format!(r#"Cannot read "{}" environment variable"#, $env_name), panic);
    };

    // Env with default value
    ($env_name:expr, $default:expr) => {
        __itconfig_variable_helper!($env_name, $default, setenv);
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_env_or_invalid_syntax!();
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_env_or_invalid_syntax {
    () => {
        compile_error!(
            "Invalid `env_or!` syntax. Please see the `env_or!` macro docs for more info.\
            `https://docs.rs/itconfig/latest/itconfig/macro.env_or.html`"
        );
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __itconfig_variable_helper {
    // Get env variable
    ($env_name:expr, $default:expr, $token:tt) => {{
        use std::env;
        use itconfig::EnvValue;
        env::var($env_name)
            .map(|val| __itconfig_variable_helper!(val))
            .unwrap_or_else(|_| __itconfig_variable_helper!(@$token $env_name, $default))
    }};

    // Returns converted env variable
    ($(@default $env_name:expr,)? $default:expr) => {{
        EnvValue::from($default.to_string()).into()
    }};

    // Set default value for env variable and returns default
    (@setenv $env_name:expr, $default:expr) => {{
        env::set_var($env_name, $default.to_string());
        __itconfig_variable_helper!($default)
    }};

    // Make panic for env variable
    (@panic $env_name:expr, $default:expr) => {
        panic!($default);
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_env_or_invalid_syntax!();
    };
}

