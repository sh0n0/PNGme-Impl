use std::fs;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(args.path.as_path())?;
    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());

    png.append_chunk(chunk);

    fs::write(args.path.as_path(), png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(args.path.as_path())?;
    let chunk_type = args.chunk_type.to_string();

    if let Some(chunk) = png.chunk_by_type(&chunk_type) {
        println!("{}", chunk.data_as_string()?);
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(args.path.as_path())?;
    let chunk_type = args.chunk_type.to_string();

    png.remove_chunk(&chunk_type)?;

    fs::write(args.path.as_path(), png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(args.path.as_path())?;

    for ele in png.chunks() {
        println!("{}", ele);
    }
    Ok(())
}
