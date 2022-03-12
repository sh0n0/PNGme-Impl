use args::PngMeArgs;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = &'static str;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();
    match args.commands {
        args::PngMeCommands::Encode(_) => {
            println!("encode")
        }
        args::PngMeCommands::Decode(_) => {
            println!("decode")
        }
        args::PngMeCommands::Remove(_) => {
            println!("remove")
        }
        args::PngMeCommands::Print(_) => {
            println!("print")
        }
    }
    Ok(())
}
