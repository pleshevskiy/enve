mod test_case_1 {
    itconfig::config! {
        MISS_VARIABLE: bool,
    }

    #[test]
    #[should_panic(expected = "Environment variable \"MISS_VARIABLE\" is missing")]
    fn should_panic_if_miss_env_variable() {
        config::init();
    }
}

mod test_case_2 {
    use std::env;

    itconfig::config! {
        DEBUG: bool,
    }

    #[test]
    fn one_variable() {
        env::set_var("DEBUG", "t");

        config::init();
        assert_eq!(config::DEBUG(), true);
        env::remove_var("DEBUG");
    }
}

mod test_case_3 {
    itconfig::config! {
        DEBUG: bool => true,
    }

    #[test]
    fn one_variable_with_default_value() {
        config::init();
        assert_eq!(config::DEBUG(), true);
    }
}

mod test_case_4 {
    itconfig::config! {
        FOO: bool => true,
        BAR: bool => false,
    }

    #[test]
    fn few_variables_with_default_value() {
        config::init();
        assert_eq!(config::FOO(), true);
        assert_eq!(config::BAR(), false);
    }
}

mod test_case_5 {
    itconfig::config! {
        NUMBER: i32 => 30,
        BOOL: bool => true,
        STR: String => "str",
        STRING: String => "string".to_string(),
    }

    #[test]
    fn different_types_with_default_value() {
        config::init();
        assert_eq!(config::NUMBER(), 30);
        assert_eq!(config::BOOL(), true);
        assert_eq!(config::STR(), "str".to_string());
        assert_eq!(config::STRING(), "string".to_string());
    }
}

mod test_case_6 {
    use std::env;

    itconfig::config! {
        T_BOOL: bool,
        TRUE_BOOL: bool,
        NUM_BOOL: bool,
        ON_BOOL: bool,
        CAMEL_CASE: bool,
        FALSE_BOOL: bool,
    }

    #[test]
    fn convert_bool_type_value_from_env() {
        env::set_var("T_BOOL", "t");
        env::set_var("TRUE_BOOL", "true");
        env::set_var("NUM_BOOL", "1");
        env::set_var("ON_BOOL", "on");
        env::set_var("CAMEL_CASE", "True");
        env::set_var("FALSE_BOOL", "false");

        config::init();
        assert_eq!(config::T_BOOL(), true);
        assert_eq!(config::TRUE_BOOL(), true);
        assert_eq!(config::NUM_BOOL(), true);
        assert_eq!(config::ON_BOOL(), true);
        assert_eq!(config::CAMEL_CASE(), true);
        assert_eq!(config::FALSE_BOOL(), false);
    }
}

mod test_case_7 {
    use std::env;

    itconfig::config! {
        I8: i8,
        I16: i16,
        I32: i32,
        I64: i64,
        I128: i128,
        ISIZE: isize,
        U8: u8,
        U16: u16,
        U32: u32,
        U64: u64,
        U128: u128,
        USIZE: usize,
        F32: f32,
        F64: f64,
    }

    #[test]
    fn convert_number_type_value_from_env() {
        env::set_var("I8", "10");
        env::set_var("I16", "10");
        env::set_var("I32", "10");
        env::set_var("I64", "10");
        env::set_var("I128", "10");
        env::set_var("ISIZE", "10");
        env::set_var("U8", "10");
        env::set_var("U16", "10");
        env::set_var("U32", "10");
        env::set_var("U64", "10");
        env::set_var("U128", "10");
        env::set_var("USIZE", "10");
        env::set_var("F32", "10");
        env::set_var("F64", "10");

        config::init();
        assert_eq!(config::I8(), 10);
        assert_eq!(config::I16(), 10);
        assert_eq!(config::I32(), 10);
        assert_eq!(config::I64(), 10);
        assert_eq!(config::ISIZE(), 10);
        assert_eq!(config::U8(), 10);
        assert_eq!(config::U16(), 10);
        assert_eq!(config::U32(), 10);
        assert_eq!(config::U64(), 10);
        assert_eq!(config::USIZE(), 10);
        assert_eq!(config::F32(), 10.0);
        assert_eq!(config::F64(), 10.0);
    }
}

mod test_case_8 {
    itconfig::config! {
        #![config(name = "custom_config_name")]

        DEBUG: bool => true,
    }

    #[test]
    fn change_configuration_module_name() {
        custom_config_name::init();
        assert_eq!(custom_config_name::DEBUG(), true);
    }
}

mod test_case_9 {
    use std::env;

    itconfig::config! {
        DEBUG: bool => true,

        DB {
            HOST: bool,
            PORT: bool => true,
            USERNAME: bool => true,
        }

        APP {}
    }

    #[test]
    fn configuration_with_namespace() {
        env::set_var("DB_HOST", "t");

        config::init();
        assert_eq!(config::DEBUG(), true);
        assert_eq!(config::DB::HOST(), true);
    }
}

mod test_case_10 {
    itconfig::config! {
        FIRST {
            SECOND {
                THIRD {
                    FOO: u32 => 50,
                }
            }
        }
    }

    #[test]
    fn configuration_with_nested_namespaces() {
        config::init();
        assert_eq!(config::FIRST::SECOND::THIRD::FOO(), 50);
    }
}

mod test_case_11 {
    itconfig::config! {
        FIRST {
            #[cfg(feature = "meta_namespace")]
            SECOND {
                THIRD {
                    FOO: u32 => 50,
                }
            }
        }
    }

    #[cfg(feature = "meta_namespace")]
    #[test]
    fn configuration_namespaces_with_custom_meta() {
        config::init();
        assert_eq!(config::FIRST::SECOND::THIRD::FOO(), 50);
    }
}

mod test_case_12 {
    use std::env;

    itconfig::config! {
        testing: bool,

        namespace {
            foo: bool,
        }
    }

    #[test]
    fn configuration_variables_and_namespace_in_lowercase() {
        env::set_var("TESTING", "t");
        env::set_var("NAMESPACE_FOO", "t");

        config::init();
        assert_eq!(config::testing(), true);
        assert_eq!(config::namespace::foo(), true);
    }
}

mod test_case_13 {
    use std::env;

    itconfig::config! {
        #[env_name = "MY_CUSTOM_NAME"]
        PER_PAGE: i32,

        APP {
            #[env_name = "MY_CUSTOM_NAME"]
            RECIPES_PER_PAGE: i32,
        }
    }

    #[test]
    fn custom_environment_name_for_variable() {
        env::set_var("MY_CUSTOM_NAME", "95");

        config::init();
        assert_eq!(config::PER_PAGE(), 95);
        assert_eq!(config::APP::RECIPES_PER_PAGE(), 95);
    }
}

mod test_case_14 {
    use std::env;

    itconfig::config! {
        #[cfg(feature = "postgres")]
        #[env_name = "MY_CUSTOM_NAME"]
        DATABASE_URL: String,

        #[cfg(not(feature = "postgres"))]
        #[env_name = "MY_CUSTOM_NAME"]
        DATABASE_URL: i32,
    }

    #[test]
    fn stranger_meta_data() {
        env::set_var("MY_CUSTOM_NAME", "95");

        config::init();
        #[cfg(not(feature = "postgres"))]
        assert_eq!(config::DATABASE_URL(), 95);

        #[cfg(feature = "postgres")]
        assert_eq!(config::DATABASE_URL(), "95");
    }
}

mod test_case_15 {
    use std::env;

    itconfig::config! {
        DEFAULT_ENV_STRING: String => "localhost",
        DEFAULT_ENV_BOOLEAN: bool => true,
        DEFAULT_ENV_UINT: u32 => 40,
        DEFAULT_ENV_FLOAT: f64 => 40.9,
    }

    #[test]
    fn setting_default_env_variable() {
        config::init();

        assert_eq!(env::var("DEFAULT_ENV_STRING"), Ok("localhost".to_string()));
        assert_eq!(env::var("DEFAULT_ENV_BOOLEAN"), Ok("true".to_string()));
        assert_eq!(env::var("DEFAULT_ENV_UINT"), Ok("40".to_string()));
        assert_eq!(env::var("DEFAULT_ENV_FLOAT"), Ok("40.9".to_string()));
    }
}

mod test_case_16 {
    use std::env;

    itconfig::config! {
        DATABASE_URL < (
            "postgres://",
            POSTGRES_USERNAME,
            ":",
            POSTGRES_PASSWORD,
            "@",
            POSTGRES_HOST,
            "/",
            POSTGRES_DB,
        ),
    }

    #[test]
    fn concatenate_environment_variables() {
        env::set_var("POSTGRES_USERNAME", "user");
        env::set_var("POSTGRES_PASSWORD", "pass");
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_DB", "test");

        config::init();
        assert_eq!(
            config::DATABASE_URL(),
            String::from("postgres://user:pass@localhost/test")
        );
    }
}

mod test_case_17 {
    use std::env;

    itconfig::config! {
        DEFAULT_CONCAT_ENV < (
            "string",
            "/",
            SETTING_DEFAULT_CONCAT_ENV_VARIABLE,
        ),
    }

    #[test]
    fn setting_default_concat_env_variable() {
        env::set_var("SETTING_DEFAULT_CONCAT_ENV_VARIABLE", "custom");

        config::init();
        assert_eq!(
            env::var("DEFAULT_CONCAT_ENV"),
            Ok("string/custom".to_string())
        );
    }
}

mod test_case_18 {
    itconfig::config! {
        DATABASE_URL < (
            "postgres://",
            PG_USERNAME,
            ":",
            PG_PASSWORD,
            "@",
            PG_HOST,
            "/",
            PG_DB,
        ),
    }

    #[test]
    #[should_panic(expected = "Environment variable \"PG_USERNAME\" is missing")]
    fn concatenate_not_defined_environment_variables() {
        config::init();
    }
}

mod test_case_19 {
    use std::env;

    itconfig::config! {
        CONCATENATED_DATABASE_URL < (
            "postgres://",
            NOT_DEFINED_PG_USERNAME => "user",
            ":",
            NOT_DEFINED_PG_PASSWORD => "pass",
            "@",
            NOT_DEFINED_PG_HOST => "localhost:5432",
            "/",
            NOT_DEFINED_PG_DB => "test",
        ),
    }

    #[test]
    fn default_value_for_concatenate_env_parameter() {
        config::init();
        assert_eq!(
            env::var("CONCATENATED_DATABASE_URL"),
            Ok("postgres://user:pass@localhost:5432/test".to_string())
        );
    }
}

mod test_case_20 {
    use std::env;
    use std::env::VarError;

    itconfig::config! {
        #[env_name = "CUSTOM_CONCAT_ENVNAME"]
        CONCAT_ENVVAR < (
            "postgres://",
            NOT_DEFINED_PG_USERNAME => "user",
            ":",
            NOT_DEFINED_PG_PASSWORD => "pass",
            "@",
            NOT_DEFINED_PG_HOST => "localhost:5432",
            "/",
            NOT_DEFINED_PG_DB => "test",
        ),
    }

    #[test]
    fn envname_meta_for_concatenated_env_variable() {
        config::init();
        assert_eq!(
            env::var("CUSTOM_CONCAT_ENVNAME"),
            Ok("postgres://user:pass@localhost:5432/test".to_string())
        );
        assert_eq!(env::var("CONCAT_ENVVAR"), Err(VarError::NotPresent));
    }
}

mod test_case_21 {
    use std::env;
    use std::env::VarError;

    itconfig::config! {
        CONCATED_NAMESPACE {
            CONCAT_ENVVAR < (
                "postgres://",
                NOT_DEFINED_PG_USERNAME => "user",
                ":",
                NOT_DEFINED_PG_PASSWORD => "pass",
                "@",
                NOT_DEFINED_PG_HOST => "localhost:5432",
                "/",
                NOT_DEFINED_PG_DB => "test",
            ),
        }
    }

    #[test]
    fn concatenated_environment_variable_in_namespace() {
        config::init();
        assert_eq!(
            env::var("CONCATED_NAMESPACE_CONCAT_ENVVAR"),
            Ok("postgres://user:pass@localhost:5432/test".to_string())
        );
        assert_eq!(env::var("CONCAT_ENVVAR"), Err(VarError::NotPresent));
    }
}

mod test_case_22 {
    itconfig::config! {
        static STATIC_STR => "test",
        static STATIC_STRING: String => "test",
        static STATIC_I8: i8 => 1,
        static STATIC_I16: i16 => 1,
        static STATIC_I32: i32 => 1,
        static STATIC_I64: i64 => 1,
        static STATIC_I128: i128 => 1,
        static STATIC_ISIZE: isize => 1,
        static STATIC_U8: u8 => 1,
        static STATIC_U16: u16 => 1,
        static STATIC_U32: u32 => 1,
        static STATIC_U64: u64 => 1,
        static STATIC_U128: u128 => 1,
        static STATIC_USIZE: usize => 1,
        static STATIC_F32: f32 => 1,
        static STATIC_F64: f64 => 1,
        static STATIC_CONCAT_VARIABLE < (
            "static ",
            STATIC_CONCAT_PART => "part",
        ),
    }

    #[test]
    fn static_variables() {
        config::init();

        assert_eq!(config::STATIC_STR(), "test");
        assert_eq!(config::STATIC_STRING(), "test".to_string());
        assert_eq!(config::STATIC_I8(), 1);
        assert_eq!(config::STATIC_I16(), 1);
        assert_eq!(config::STATIC_I32(), 1);
        assert_eq!(config::STATIC_I64(), 1);
        assert_eq!(config::STATIC_I128(), 1);
        assert_eq!(config::STATIC_ISIZE(), 1);
        assert_eq!(config::STATIC_U8(), 1);
        assert_eq!(config::STATIC_U16(), 1);
        assert_eq!(config::STATIC_U32(), 1);
        assert_eq!(config::STATIC_U64(), 1);
        assert_eq!(config::STATIC_U128(), 1);
        assert_eq!(config::STATIC_USIZE(), 1);
        assert_eq!(config::STATIC_F32(), 1.0);
        assert_eq!(config::STATIC_F64(), 1.0);
        assert_eq!(config::STATIC_CONCAT_VARIABLE(), "static part".to_string())
    }
}

mod test_case_23 {
    use std::env;

    itconfig::config! {
        SOMETHING: Option<&'static str>,
        #[env_name = "SOMETHING"]
        STD_SOMETHING: std::option::Option<&'static str>,
        #[env_name = "SOMETHING"]
        CORE_SOMETHING: core::option::Option<&'static str>,

        NOTHING: Option<&'static str>,
    }

    #[test]
    fn optional_variables() {
        env::set_var("SOMETHING", "hello world");

        assert_eq!(config::SOMETHING(), Some("hello world"));
        assert_eq!(config::STD_SOMETHING(), Some("hello world"));
        assert_eq!(config::CORE_SOMETHING(), Some("hello world"));
        assert_eq!(config::NOTHING(), None);
    }
}

mod test_case_24 {
    use std::env;

    itconfig::config! {
        MY_VEC: Vec<&'static str>,
        #[env_name = "MY_VEC"]
        STD_VEC: std::vec::Vec<&'static str>,
    }

    #[test]
    fn vector_of_values() {
        env::set_var("MY_VEC", "paypal,stripe");

        assert_eq!(config::MY_VEC(), vec!["paypal", "stripe"]);
        assert_eq!(config::STD_VEC(), vec!["paypal", "stripe"]);
    }
}
