use itconfig::EnvError::*;
use itconfig::*;
use std::env;

#[test]
#[should_panic(expected = "Environment variable \"TEST_CASE_1\" is missing")]
fn get_missing_env() {
    get_env_or_panic::<String>("TEST_CASE_1");
}

#[test]
#[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_2\"")]
fn get_env_with_invalid_value() {
    let env_name = "TEST_CASE_2";
    env::set_var(&env_name, "30r");
    get_env_or_panic::<u32>(env_name);
}

#[test]
fn get_result_of_missing_env() {
    let env_name = String::from("TEST_CASE_3");
    let env_val = get_env::<String>(&env_name);
    assert_eq!(env_val, Err(MissingVariable { env_name }))
}

#[test]
fn get_result_of_env_with_invalid_value() {
    let env_name = String::from("TEST_CASE_4");
    env::set_var(&env_name, "30r");
    let env_val = get_env::<u32>(&env_name);
    assert_eq!(env_val, Err(FailedToParse { env_name }))
}

#[test]
fn get_result_of_env_successfully() {
    env::set_var("TEST_CASE_5", "30");
    let env_var = get_env("TEST_CASE_5");
    assert_eq!(env_var, Ok(30));
}

#[test]
fn get_missing_env_with_default_value() {
    let flag: bool = get_env_or_default("TEST_CASE_6", "true");
    assert_eq!(flag, true);
}

#[test]
#[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_7\"")]
fn get_invalid_env_with_default_value() {
    env::set_var("TEST_CASE_7", "30r");
    get_env_or_default::<u32, _>("TEST_CASE_7", 30);
}

#[test]
#[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_8\"")]
fn get_env_with_invalid_default_value() {
    get_env_or_default::<u32, _>("TEST_CASE_8", "30r");
}

#[test]
fn get_env_with_default_successfully() {
    env::set_var("TEST_CASE_9", "10");
    let env_val: u32 = get_env_or_default("TEST_CASE_9", 30);
    assert_eq!(env_val, 10)
}

#[test]
fn get_missing_env_with_set_default_value() {
    let flag: bool = get_env_or_set_default("TEST_CASE_10", "true");
    assert_eq!(flag, true);

    let env_var = env::var("TEST_CASE_10");
    assert_eq!(env_var, Ok(String::from("true")))
}
