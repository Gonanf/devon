use anyhow::{Context, Ok, Result};
use clap::Parser;
use iroh::{protocol::Router, Endpoint, SecretKey, Watcher};
use iroh_blobs::{
    api::{
        blobs::{self, AddPathOptions, ExportMode, ExportOptions, ImportMode},
        remote::GetProgressItem,
        tags::TagInfo, Store,
    }, format::collection::Collection, get::{self, Stats}, store::{fs::FsStore, mem::MemStore}, ticket::{self, BlobTicket}, BlobFormat, BlobsProtocol, Hash
};
use n0_future::{BufferedStreamExt, IterExt, StreamExt};
use std::{
    ffi::OsString,
    fs::{self, create_dir_all},
    path::{self, Path, PathBuf},
};
use tokio::{
    fs::{create_dir, remove_dir_all},
    sync::mpsc,
};
use walkdir::WalkDir;

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
