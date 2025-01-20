mod args;
mod chunk;
mod chunk_type;
mod commands;
mod crc;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    args::parse();
    Ok(())
}
