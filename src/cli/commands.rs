use std::path::PathBuf;
use clap::{Parser, Subcommand};
use iroh_blobs::ticket::BlobTicket;

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
