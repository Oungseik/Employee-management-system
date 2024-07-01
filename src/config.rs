#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env, default_value = "8989")]
    pub port: u16,
}
