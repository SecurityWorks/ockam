use clap::Args;
use miette::IntoDiagnostic;

use ockam::Context;
use ockam_api::nodes::InMemoryNode;
use ockam_api::orchestrator::project::ProjectsOrchestratorApi;
use ockam_api::output::Output;

use crate::shared_args::IdentityOpts;
use crate::{docs, CommandGlobalOpts};

/// Show project details
#[derive(Clone, Debug, Args)]
#[command(hide = docs::hide())]
pub struct InfoCommand {
    /// Name of the project.
    #[arg(default_value = "default")]
    pub name: String,

    #[command(flatten)]
    pub identity_opts: IdentityOpts,
}

impl InfoCommand {
    pub fn name(&self) -> String {
        "project information".into()
    }

    pub async fn run(&self, ctx: &Context, opts: CommandGlobalOpts) -> miette::Result<()> {
        let node = InMemoryNode::start(ctx, &opts.state).await?;
        let project = node.get_project_by_name(ctx, &self.name).await?;
        opts.terminal
            .stdout()
            .plain(project.item()?)
            .json(serde_json::to_string(&project).into_diagnostic()?)
            .write_line()?;
        Ok(())
    }
}
