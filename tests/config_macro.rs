use std::env;
#[macro_use]
extern crate it_config;

#[test]
fn one_variable() {
    config! {
        DEBUG: bool => true,
    }

    assert_eq!(cfg::DEBUG(), true);
}

#[test]
fn few_variables() {
    config! {
        FOO: bool => true,
        BAR: bool => false,
    }

    assert_eq!(cfg::FOO(), true);
    assert_eq!(cfg::BAR(), false);
}

#[test]
fn different_types() {
    config! {
        NUMBER: i32 => 30,
        BOOL: bool => true,
        STRING: String => "string".to_string(),
    }

    assert_eq!(cfg::NUMBER(), 30);
    assert_eq!(cfg::BOOL(), true);
    assert_eq!(cfg::STRING(), "string");
}

#[test]
fn convert_bool_type_from_env() {
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

    assert_eq!(cfg::T_BOOL(), true);
    assert_eq!(cfg::TRUE_BOOL(), true);
    assert_eq!(cfg::NUM_BOOL(), true);
    assert_eq!(cfg::ON_BOOL(), true);
    assert_eq!(cfg::CAMEL_CASE(), true);
    assert_eq!(cfg::FALSE_BOOL(), false);
}

#[test]
fn convert_number_value_from_env() {
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
