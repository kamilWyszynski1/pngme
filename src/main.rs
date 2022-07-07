use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let cli = args::Arguments::parse();
    println!("{:?}", cli);
    cli.run()
}
