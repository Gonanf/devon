use anyhow::{Ok, Result};
use clap::Parser;

mod cli;
use cli::commands::*;
mod p2p;
use p2p::send::*;
use p2p::receive::*;

#[tokio::main]
async fn main() -> Result<()> {
    let comandos = Comandos::parse();
    match comandos.operation {
        Operation::Send { path, database} => send(path, database).await?,
        Operation::Receive { path, ticket } => receive(path, ticket).await?,
    }
    Ok(())
}
