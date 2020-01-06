use std::env;

#[macro_use]
extern crate itconfig;


#[test]
#[should_panic]
fn should_panic_if_miss_env_variable() {
    config! {
        MISS_VARIABLE: bool,
    }

    cfg::init();
}


#[test]
fn one_variable() {
    env::set_var("DEBUG", "t");

    config! {
        DEBUG: bool,
    }

    cfg::init();
    assert_eq!(cfg::DEBUG(), true);
    env::remove_var("DEBUG");
}

#[test]
fn one_variable_with_default_value() {
    config! {
        DEBUG: bool => true,
    }

    cfg::init();
    assert_eq!(cfg::DEBUG(), true);
}

#[test]
fn few_variables_with_default_value() {
    config! {
        FOO: bool => true,
        BAR: bool => false,
    }

    cfg::init();
    assert_eq!(cfg::FOO(), true);
    assert_eq!(cfg::BAR(), false);
}

#[test]
fn different_types_with_default_value() {
    config! {
        NUMBER: i32 => 30,
        BOOL: bool => true,
        STRING: String => "string".to_string(),
    }

    cfg::init();
    assert_eq!(cfg::NUMBER(), 30);
    assert_eq!(cfg::BOOL(), true);
    assert_eq!(cfg::STRING(), "string");
}

#[test]
fn convert_bool_type_value_from_env() {
    env::set_var("T_BOOL", "t");
    env::set_var("TRUE_BOOL", "true");
    env::set_var("NUM_BOOL", "1");
    env::set_var("ON_BOOL", "on");
    env::set_var("CAMEL_CASE", "True");
    env::set_var("FALSE_BOOL", "false");

    config! {
        T_BOOL: bool,
        TRUE_BOOL: bool,
        NUM_BOOL: bool,
        ON_BOOL: bool,
        CAMEL_CASE: bool,
        FALSE_BOOL: bool,

    }

    cfg::init();
    assert_eq!(cfg::T_BOOL(), true);
    assert_eq!(cfg::TRUE_BOOL(), true);
    assert_eq!(cfg::NUM_BOOL(), true);
    assert_eq!(cfg::ON_BOOL(), true);
    assert_eq!(cfg::CAMEL_CASE(), true);
    assert_eq!(cfg::FALSE_BOOL(), false);
}

#[test]
fn convert_number_type_value_from_env() {
    env::set_var("I8", "10");
    env::set_var("I16", "10");
    env::set_var("I32", "10");
    env::set_var("I64", "10");
    env::set_var("I128", "10");
    env::set_var("ISIZE","10");
    env::set_var("U8", "10");
    env::set_var("U16", "10");
    env::set_var("U32", "10");
    env::set_var("U64", "10");
    env::set_var("U128", "10");
    env::set_var("USIZE","10");
    env::set_var("F32", "10");
    env::set_var("F64","10");

    config! {
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

    cfg::init();
    assert_eq!(cfg::I8(), 10);
    assert_eq!(cfg::I16(), 10);
    assert_eq!(cfg::I32(), 10);
    assert_eq!(cfg::I64(), 10);
    assert_eq!(cfg::ISIZE(), 10);
    assert_eq!(cfg::U8(), 10);
    assert_eq!(cfg::U16(), 10);
    assert_eq!(cfg::U32(), 10);
    assert_eq!(cfg::U64(), 10);
    assert_eq!(cfg::USIZE(), 10);
    assert_eq!(cfg::F32(), 10.0);
    assert_eq!(cfg::F64(), 10.0);
}


#[test]
fn change_configuration_module_name() {
    config! {
        #![mod_name = custom_config_name]

        DEBUG: bool => true,
    }

    custom_config_name::init();
    assert_eq!(custom_config_name::DEBUG(), true);
}


#[test]
fn configuration_with_namespace() {
    env::set_var("POSTGRES_HOST", "t");

    config! {
        DEBUG: bool => true,

        POSTGRES {
            HOST: bool,
            PORT: bool => true,
            USERNAME: bool => true,
        }

        APP {}
    }

    cfg::init();
    assert_eq!(cfg::DEBUG(), true);
    assert_eq!(cfg::POSTGRES::HOST(), true);
    env::remove_var("POSTGRES_HOST");
}


#[test]
fn configuration_variables_and_namespace_in_lowercase() {
    env::set_var("TESTING", "t");
    env::set_var("NAMESPACE_FOO", "t");

    config! {
        testing: bool,

        namespace {
            foo: bool,
        }
    }

    cfg::init();
    assert_eq!(cfg::testing(), true);
    assert_eq!(cfg::namespace::foo(), true);
    env::remove_var("TESTING");
    env::remove_var("NAMESPACE_FOO");
}


#[test]
fn custom_environment_name_for_variable() {
    env::set_var("MY_CUSTOM_NAME", "95");

    config! {
        #[env_name = "MY_CUSTOM_NAME"]
        PER_PAGE: i32,

        APP {
            #[env_name = "MY_CUSTOM_NAME"]
            RECIPES_PER_PAGE: i32,
        }
    }

    cfg::init();
    assert_eq!(cfg::PER_PAGE(), 95);
    assert_eq!(cfg::APP::RECIPES_PER_PAGE(), 95);
    env::remove_var("MY_CUSTOM_NAME");
}

#[test]
fn stranger_meta_data() {
    env::set_var("MY_CUSTOM_NAME", "95");

    config! {
        #[cfg(feature = "postgres")]
        #[env_name = "MY_CUSTOM_NAME"]
        DATABASE_URL: String,

        #[cfg(not(feature = "postgres"))]
        #[env_name = "MY_CUSTOM_NAME"]
        DATABASE_URL: i32,
    }

    cfg::init();
    #[cfg(not(feature = "postgres"))]
    assert_eq!(cfg::DATABASE_URL(), 95);

    #[cfg(feature = "postgres")]
    assert_eq!(cfg::DATABASE_URL(), "95");
    env::remove_var("MY_CUSTOM_NAME");
}

#[test]
fn setting_default_env_variable() {
    config! {
        DEFAULT_ENV_STRING: String => "localhost".to_string(),
        DEFAULT_ENV_BOOLEAN: bool => true,
        DEFAULT_ENV_UINT: u32 => 40,
        DEFAULT_ENV_FLOAT: f64 => 40.9,
    }

    cfg::init();

    assert_eq!(env::var("DEFAULT_ENV_STRING"), Ok("localhost".to_string()));
    assert_eq!(env::var("DEFAULT_ENV_BOOLEAN"), Ok("true".to_string()));
    assert_eq!(env::var("DEFAULT_ENV_UINT"), Ok("40".to_string()));
    assert_eq!(env::var("DEFAULT_ENV_FLOAT"), Ok("40.9".to_string()));
}

