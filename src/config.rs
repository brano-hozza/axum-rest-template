use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,
}
