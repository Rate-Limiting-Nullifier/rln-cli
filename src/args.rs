#![allow(clippy::upper_case_acronyms)]

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::engine;

#[derive(Parser)]
#[clap(name = "Rate Limit Nullifier CLI", version = "0.1")]
#[command(about = engine::utils::RLN_ASCII)]
#[command(disable_help_subcommand = true)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Generates smart-contract")]
    Contract { depth: u8, deposit: u128 },
    #[command(about = "Generates circuit")]
    Circuit,
    #[command(about = "Generates webapp template")]
    Webapp,
    #[command(about = "Generates proof")]
    Prove { path: PathBuf },
    #[command(about = "Verifies given RLN zk-proof")]
    Verify { path: PathBuf },
}
