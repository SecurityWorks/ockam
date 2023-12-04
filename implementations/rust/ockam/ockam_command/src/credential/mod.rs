use clap::{Args, Subcommand};
use colorful::Colorful;

pub(crate) use issue::IssueCommand;
pub(crate) use verify::VerifyCommand;

use crate::output::Output;
use crate::{CommandGlobalOpts, Result};

pub(crate) mod issue;
pub(crate) mod verify;

/// Manage Credentials
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help = true, subcommand_required = true)]
pub struct CredentialCommand {
    #[command(subcommand)]
    subcommand: CredentialSubcommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum CredentialSubcommand {
    #[command(display_order = 900)]
    Issue(IssueCommand),
    Verify(VerifyCommand),
}

impl CredentialCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        match self.subcommand {
            CredentialSubcommand::Issue(c) => c.run(options),
            CredentialSubcommand::Verify(c) => c.run(options),
        }
    }
}

pub struct CredentialOutput {
    name: String,
    credential: String,
    is_verified: bool,
}

impl Output for CredentialOutput {
    fn output(&self) -> Result<String> {
        let is_verified = if self.is_verified {
            "✔︎".light_green()
        } else {
            "✕".light_red()
        };
        let output = format!(
            "Credential: {cred_name} {is_verified}\n{cred}",
            cred_name = self.name,
            is_verified = is_verified,
            cred = self.credential
        );

        Ok(output)
    }
}
