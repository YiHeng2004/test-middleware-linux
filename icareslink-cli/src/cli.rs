use std::process::ExitCode;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use console::style;

use crate::commands::{error::CliError, testing::Testing};

#[async_trait]
pub trait RunCommand {
    async fn run(self) -> Result<(), CliError>;
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Display hello world
    Testing(Testing),
}

impl Cli {
    pub async fn run(self) -> ExitCode {
        let output = match self.command {
            Commands::Testing(testing) => testing.run().await,
        };

        match output {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{}", style(e).for_stderr().red());
                ExitCode::FAILURE
            }
        }
    }
}
