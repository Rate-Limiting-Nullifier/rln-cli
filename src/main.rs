use clap::Parser;

mod args;
use args::*;

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::GenerateContract {} => println!("ava"),
    };
}
