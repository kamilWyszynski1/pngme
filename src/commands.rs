use anyhow::{anyhow, Context, Ok};
use std::{
    fs::{read, read_to_string, write, File},
    str::FromStr,
};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};

pub fn encode(
    file_path: String,
    chunk_type: String,
    message: String,
    output_file: Option<String>,
) -> Result<()> {
    let file_bytes = read(&file_path)?;
    let mut png = Png::try_from(&file_bytes[..])?;
    let chunk = Chunk::new(ChunkType::from_str(&chunk_type)?, message.into_bytes());

    png.append_chunk(chunk);

    let write_path = output_file.unwrap_or(file_path);

    write(write_path, png.as_bytes())?;
    Ok(())
}

pub fn decode(file_path: String, chunk_type: String) -> Result<String> {
    let file_bytes = read(&file_path)?;
    let png = Png::try_from(&file_bytes[..])?;

    png.chunk_by_type(&chunk_type)
        .context("not chunk type")?
        .data_as_string()
}

pub fn remove(file_path: String, chunk_type: String) -> Result<String> {
    let file_bytes = read(&file_path)?;
    let mut png = Png::try_from(&file_bytes[..])?;
    let removed = png
        .remove_chunk(&chunk_type)
        .map(|chunk| chunk.to_string())?;

    write(file_path, png.as_bytes())?;
    Ok(removed)
}

pub fn print(file_path: String) -> Result<()> {
    let file_bytes = read(&file_path)?;
    let png = Png::try_from(&file_bytes[..])?;

    print!("{:?}", png);

    Ok(())
}
