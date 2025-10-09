use super::*;
use serial_test::serial;
use tools::EnvSetter;

#[test]
#[serial]
fn test_config_load_all_values() {
    let mut env = EnvSetter::new();
    env.set("SPELL_LOG_LEVEL", "debug");
    let config = load();
    assert_eq!(config.log_level, "debug");
}

#[test]
#[serial]
fn test_config_load_defaults() {
    let _env = EnvSetter::new();
    let config = load();
    assert_eq!(config.log_level, "info");
}
