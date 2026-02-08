use anyhow::Result;
use clap::Parser;
use mcp_server_roundtable::{
    config::{ConfigLoader, VibeConfig},
    server::VibeServer,
    session_store::SessionStore,
};
use rmcp::{transport::stdio, ServiceExt};
use std::path::PathBuf;

/// Roundtable MCP router: multi-LLM, session-aware delegator.
#[derive(Parser, Debug)]
#[command(
    name = "mcp-server-roundtable",
    version,
    about = "MCP server routing prompts to multiple local LLM CLIs with session reuse",
    long_about = None
)]
struct Cli {
    /// Optional config file path (JSON). If omitted, uses ~/.config/roundtable/config.json.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Optional session store path (JSON). If omitted, uses ~/.local/share/roundtable/sessions.json.
    #[arg(long)]
    sessions: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Config precedence is implemented per-request in `ConfigLoader`:
    // - user config: ~/.config/roundtable/config.json (or --config)
    // - project override: <repo>/.roundtable/config.json or <repo>/.roundtable.json
    let user_cfg_path = cli.config.or_else(VibeConfig::default_path);
    let loader = ConfigLoader::new(user_cfg_path);

    let store_path = cli.sessions.unwrap_or_else(SessionStore::default_path);
    let store = SessionStore::new(store_path);

    let service = VibeServer::new(loader, store)
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("serving error: {e:?}");
        })?;

    service.waiting().await?;
    Ok(())
}
