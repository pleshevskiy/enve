//! # itconfig
//!
//! Simple configuration with macro for rust application.
//!
//!
//! ## Example usage
//!
//! ```rust
//! #[macro_use] extern crate itconfig;
//! // use dotenv::dotenv;
//!
//! config! {
//!     DEBUG: bool => true,
//!     HOST: String => "127.0.0.1".to_string(),
//!
//!     NAMESPACE {
//!         FOO: bool => true,
//!         BAR: i32 => 10,
//!     }
//! }
//!
//! fn main () {
//!     // dotenv().ok();
//!     cfg::init();
//!     assert_eq!(cfg::HOST(), String::from("127.0.0.1"));
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
///
/// This module will also contain helper method:
///
/// `init`
/// ------
///
/// If you miss some required variables your application will panic at startup.
/// Run this at the main function for check all required variables without default value.
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
#[macro_export]
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

    // Find variable with default value
    (
        tokens = [
            $name:ident : $ty:ty => $default:expr,
            $($rest:tt)*
        ],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                name = $name,
                ty = $ty,
                default = $default,
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // Find variable without default value
    (
        tokens = [
            $name:ident : $ty:ty,
            $($rest:tt)*
        ],
        $($args:tt)*
    ) => {
        __itconfig_parse_variables! {
            current_variable = {
                name = $name,
                ty = $ty,
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // Done parsing variable
    (
        current_variable = {
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
            name = $var_name:ident,
            $($variable:tt)*
        },)*],
        namespaces = [$({
            variables = [$({
                name = $ns_var_name:tt,
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
            use std::env;
            use $crate::EnvValue;

            $(
                pub mod $ns_name {
                    use std::env;
                    use $crate::EnvValue;

                    $(__itconfig_variable! {
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
    (
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        ty = $ty:ty,
        $($args:tt)*
    ) => {
        __itconfig_variable! {
            name = $name,
            env_prefix = $env_prefix,
            env_name = concat!($env_prefix, stringify!($name)).to_uppercase(),
            ty = $ty,
            $($args)*
        }
    };

    // Add method without default value
    (
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        env_name = $env_name:expr,
        ty = $ty:ty,
    ) => {
        __itconfig_variable! {
            name = $name,
            env_prefix = $env_prefix,
            env_name = $env_name,
            ty = $ty,
            default = panic!(format!(r#"Cannot read "{}" environment variable"#, $env_name)),
        }
    };

    // Add method with default value
    (
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        env_name = $env_name:expr,
        ty = $ty:ty,
        default = $default:expr,
    ) => {
        pub fn $name() -> $ty {
            env::var($env_name)
                .map(|val| EnvValue::from(val).into())
                .unwrap_or_else(|_| $default)
        }
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}

