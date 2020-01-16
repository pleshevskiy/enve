use std::env;
use itconfig::*;

#[test]
fn missing_env_variable() {
    let flag: bool = get_env_or_default("DEFAULT_TEST_CASE_1", "true");
    assert_eq!(flag, true);

//    let var: String = env::var("DEFAULT_TEST_CASE_1").unwrap();
//    assert_eq!(var, "true");
}

#[test]
#[should_panic(expected = "Failed to parse environment variable \"DEFAULT_TEST_CASE_2\"")]
fn cannot_parse_env_variable() {
    env::set_var("DEFAULT_TEST_CASE_2", "30r");
    let _: u32 = get_env_or_default("DEFAULT_TEST_CASE_2", 30);
}

#[test]
#[should_panic(expected = "Failed to parse environment variable \"DEFAULT_TEST_CASE_2\"")]
fn cannot_parse_default_value() {
    let _: u32 = get_env_or_default("DEFAULT_TEST_CASE_2", "30r");
}

#[test]
fn get_env_successfully() {
    let a: u32 = get_env_or_default("DEFAULT_TEST_CASE_3", 30);

    assert_eq!(a, 30);
}
