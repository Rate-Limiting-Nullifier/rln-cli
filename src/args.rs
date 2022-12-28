#![allow(clippy::upper_case_acronyms)]

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Rate Limiting Nullifier CLI", version = "0.1")]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    GenerateContract,
    GenerateProof { path: PathBuf },
    VerifyProof { path: PathBuf },
}
