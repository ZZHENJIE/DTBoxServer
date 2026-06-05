use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "DTBox",
    version,
    about = "A stock data platform with real-time market data, screening tools, and user management.",
    long_about = None
)]

pub struct Cli {
    #[arg(short, long, default_value = "./config.json")]
    pub config: String, // Config file path
}
