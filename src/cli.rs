use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "project", about = "A Rust CLI using clap", version)]
pub struct Cli {
    /// Increase logging verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Command1 {
        #[arg(long, help = "First argument")]
        arg1: String,
        #[arg(short = 's', long, help = "Second argument")]
        arg2: String,
    },
    Command2 {
        #[arg(long = "input", help = "Input file")]
        arg1: String,
    },
    Command3 {
        #[arg(long)]
        arg1: String,
        #[arg(short, long)]
        arg3: String,
        #[arg(short = 'o', long = "output")]
        arg4: String,
    },
}