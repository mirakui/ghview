mod ipc;
mod mcp;

use anyhow::Result;
use mcp::McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    let server = McpServer::new().await?;
    server.run().await?;
    Ok(())
}
