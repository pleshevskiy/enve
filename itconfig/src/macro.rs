
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
macro_rules! __itconfig_get_ty_or_default {
    () => { &'static str };
    ($ty:ty) => { $ty };
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

    // Find static concatenated variable
    (
        tokens = [
            $(#$meta:tt)*
            static $name:ident < ($($inner:tt)+),
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
                is_static = true,
            },
            tokens = [$($rest)*],
            $($args)*
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
                is_static = false,
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // Find static variable
    (
        tokens = [
            $(#$meta:tt)*
            static $name:ident $(: $ty:ty)? $(=> $default:expr)?,
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
                ty = __itconfig_get_ty_or_default!($($ty)?),
                is_static = true,
                $(default = $default,)?
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // Find variable
    (
        tokens = [
            $(#$meta:tt)*
            $name:ident $(: $ty:ty)? $(=> $default:expr)?,
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
                ty = __itconfig_get_ty_or_default!($($ty)?),
                is_static = false,
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
            $(env_name = $env_name:expr,)?
            ty = $ty:ty,
            is_static = $is_static:ident,
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
            #[cfg(feature = "static")]
            use lazy_static::lazy_static;

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
                $(env_name = $env_name,)?
                ty = $ty,
                is_static = $is_static,
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
    ($env_name:ident => $default:expr) => (
        itconfig::get_env_or_default(
            stringify!($env_name).to_uppercase().as_str(),
            $default
        )
    );

    // Find env parameter without default value
    ($env_name:ident) => (
        itconfig::get_env_or_panic(stringify!($env_name).to_uppercase().as_str())
    );

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

    // Add method for env variable
    (
        meta = $meta:tt,
        concat = $concat:tt,
        name = $name:ident,
        env_prefix = $env_prefix:expr,
        env_name = $env_name:expr,
        ty = $ty:ty,
        is_static = $is_static:ident,
        $(default = $default:expr,)?
    ) => {
        __itconfig_variable!(
            @wrap
            is_static = $is_static,
            meta = $meta,
            name = $name,
            ty = $ty,
            value = __itconfig_variable!(
                @inner
                concat = $concat,
                env_name = $env_name,
                $(default = $default,)?
            ),
        );
    };

    // Wrap static variables
    (
        @wrap
        is_static = true,
        meta = [$(#$meta:tt,)*],
        name = $name:ident,
        ty = $ty:ty,
        value = $value:expr,
    ) => (
        $(#$meta)*
        pub fn $name() -> $ty {
            lazy_static! {
                static ref $name: $ty = $value;
            }

            (*$name).clone()
        }
    );

    // Wrap functions
    (
        @wrap
        is_static = false,
        meta = [$(#$meta:tt,)*],
        name = $name:ident,
        ty = $ty:ty,
        value = $value:expr,
    ) => (
        $(#$meta)*
        pub fn $name() -> $ty { $value }
    );

    // Concatenate function body
    (
        @inner
        concat = [$($concat:expr,)+],
        env_name = $env_name:expr,
        $($args:tt)*
    ) => ({
        let value_parts: Vec<String> = vec!($($concat),+);
        let value = value_parts.join("");
        std::env::set_var($env_name, value.as_str());
        value
    });

    // Env without default
    (
        @inner
        concat = [],
        env_name = $env_name:expr,
    ) => (
        itconfig::get_env_or_panic($env_name.to_string().as_str())
    );

    // Env with default
    (
        @inner
        concat = [],
        env_name = $env_name:expr,
        default = $default:expr,
    ) => (
        itconfig::get_env_or_set_default(
            $env_name.to_string().as_str(),
            $default
        )
    );

    // Invalid syntax
    ($($tokens:tt)*) => {
        __itconfig_invalid_syntax!();
    };
}

