use async_trait::async_trait;
use clap::Args;
use console::style;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use icareslink_controller::proto::Message;

use crate::cli::RunCommand;

use super::error::CliError;

#[derive(Args, Debug)]
pub struct Testing {
    pub message: Option<String>,
}

#[async_trait]
impl RunCommand for Testing {
    async fn run(self) -> Result<(), CliError> {

        let message = match self.message {
            Some(message) => message,
            None => Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Insert your message:")
                .interact_text()?,
        };
        println!("{}", style("This is testing function of calling daemon by cli").yellow());
        println!("{}", style(&format!("Your message is: {}", message)).yellow());

        let mut client = icareslink_controller::new_grpc_client()
            .await
            .map_err(|_| CliError::DaemonUnavailable)?;

        client
            .print_hello(Message {message}).await?;

        Ok(())
    }
}
