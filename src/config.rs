use clap::Parser;
use std::sync::OnceLock;

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env, default_value = "8989")]
    pub port: u16,
    #[clap(long, env, default_value = "")]
    pub email_domain: String,
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| Config::parse())
}
