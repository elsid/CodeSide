#[cfg(feature = "read_config")]
#[test]
fn test_read_config() {
    use aicup2019::my_strategy::Config;

    let config: Config = rustc_serialize::json::decode(
        std::fs::read_to_string(
            "etc/config.json"
        ).expect("Can't read config file").as_str()
    ).expect("Can't parse config file");

    assert_eq!(config, Config::new());
}
