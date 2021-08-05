use async_trait::async_trait;
use clap::Clap;
use miette::Diagnostic;
use nuget_api::v3::NuGetClient;
use ruget_command::RuGetCommand;
use ruget_config::RuGetConfigLayer;
use thiserror::Error;

#[derive(Debug, Clap, RuGetConfigLayer)]
pub struct UnlistCmd {
    #[clap(about = "ID of package to unlist")]
    id: String,
    #[clap(about = "Version of package to unlist")]
    version: String,
    #[clap(
        about = "Source for package",
        default_value = "https://api.nuget.org/v3/index.json",
        long
    )]
    source: String,
    #[clap(from_global)]
    loglevel: log::LevelFilter,
    #[clap(from_global)]
    quiet: bool,
    #[clap(from_global)]
    json: bool,
    #[clap(from_global)]
    api_key: Option<String>,
}

#[async_trait]
impl RuGetCommand for UnlistCmd {
    async fn execute(self) -> Result<(), Box<dyn Diagnostic + Send + Sync + 'static>> {
        let client = NuGetClient::from_source(self.source.clone())
            .await?
            .with_key(self.api_key);
        client.unlist(self.id.clone(), self.version.clone()).await?;
        if !self.quiet {
            println!("{}@{} has been unlisted.", self.id, self.version);
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum UnlistError {
    /// Api Key is missing.
    #[error("Missing API key")]
    MissingApiKey,
}
