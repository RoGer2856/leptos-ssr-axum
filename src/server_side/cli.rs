use clap::Parser;

#[derive(Debug, Parser)]
pub enum Commands {
    Serve,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
