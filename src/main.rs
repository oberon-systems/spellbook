mod libs;

use libs::config;
use libs::fs;

fn main() {

    // load config defaults and env
    let mut config = config::load();

    // setup logger
    env_logger::Builder::from_env(
        env_logger::Env::default()
        .default_filter_or(&config.log_level)
    )
    .init();

    // trying to load local config
    config.merge()
        .expect("failed to merge config");

    log::debug!("configuration: {:?}", config);

    fs::ensure_dirs(&config).unwrap_or_else(|e| {
        log::error!("Failed to create directories: {}", e);
        std::process::exit(1);
    });

}
