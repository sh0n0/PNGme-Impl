use args::PngMeArgs;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();
    match args.commands {
        args::PngMeCommands::Encode(args) => commands::encode(args),
        args::PngMeCommands::Decode(args) => commands::decode(args),
        args::PngMeCommands::Remove(args) => commands::remove(args),
        args::PngMeCommands::Print(args) => commands::print_chunks(args),
    }
}
