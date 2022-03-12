use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::chunk_type::ChunkType;

#[derive(Debug, Parser)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub commands: PngMeCommands,
}

#[derive(Debug, Subcommand)]
pub enum PngMeCommands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub path: PathBuf,
}
