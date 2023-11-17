use async_trait::async_trait;
use clap::Args;
use console::style;

use crate::cli::RunCommand;

use super::error::CliError;

#[derive(Args, Debug)]
pub struct Testing;

#[async_trait]
impl RunCommand for Testing {
    async fn run(self) -> Result<(), CliError> {
        println!("{}", style("Hello World!").yellow());

        print!("{}", CliError::DaemonUnavailable);

        let mut client = icareslink_controller::new_grpc_client()
            .await
            .map_err(|_| CliError::DaemonUnavailable)?;

        client.print_hello(()).await?;

        Ok(())
    }
}
