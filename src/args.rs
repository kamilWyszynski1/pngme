use crate::{
    commands::{decode, encode, print, remove},
    Result,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

impl Arguments {
    pub fn run(&self) -> Result<()> {
        self.command.clone().run()
    }
}

#[derive(Debug, Subcommand, Clone)]
enum Command {
    Encode {
        #[clap(value_parser)]
        file_path: String,

        #[clap(value_parser)]
        chunk_type: String,

        #[clap(value_parser)]
        message: String,

        #[clap(value_parser)]
        output_file: Option<String>,
    },

    Decode {
        #[clap(value_parser)]
        file_path: String,

        #[clap(value_parser)]
        chunk_type: String,
    },

    Remove {
        #[clap(value_parser)]
        file_path: String,

        #[clap(value_parser)]
        chunk_type: String,
    },

    Print {
        #[clap(value_parser)]
        file_path: String,
    },
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Command::Encode {
                file_path,
                chunk_type,
                message,
                output_file,
            } => encode(file_path, chunk_type, message, output_file),
            Command::Decode {
                file_path,
                chunk_type,
            } => decode(file_path, chunk_type).map(|msg| println!("{}", msg)),
            Command::Remove {
                file_path,
                chunk_type,
            } => remove(file_path, chunk_type).map(|msg| println!("{}", msg)),
            Command::Print { file_path } => print(file_path),
        }
    }
}
