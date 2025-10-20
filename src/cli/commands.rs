use std::{
    ffi::OsString,
    fs::{self, create_dir_all},
    path::{self, Path, PathBuf},
};
use clap::{Parser, Subcommand};
use iroh_blobs::{
    api::{
        blobs::{self, AddPathOptions, ExportMode, ExportOptions, ImportMode},
        remote::GetProgressItem,
        tags::TagInfo, Store,
    }, format::collection::Collection, get::{self, Stats}, store::{fs::FsStore, mem::MemStore}, ticket::{self, BlobTicket}, BlobFormat, BlobsProtocol, Hash
};

#[derive(Parser)]
#[command(version, about)]
pub struct Comandos {
    #[command(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
    Send {
        #[clap(long,short)]
        path: Option<PathBuf>,
        #[clap(long,short)]
        database: Option<PathBuf>,
    },
    Receive {
        ticket: BlobTicket,
        #[clap(long,short)]
        path: Option<PathBuf>,
    },
}
