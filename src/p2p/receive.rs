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
use crate::cli::commands::*;
use git2::Repository;


pub async fn receive(path: Option<PathBuf>, ticket: BlobTicket) -> Result<()> {
    let mut rng = rand::rngs::OsRng;
    let key =  SecretKey::generate(&mut rng);
    let endpoint = Endpoint::builder().secret_key(key).discovery_n0().bind().await?;

    let temp_path =
        std::env::current_dir()?.join(PathBuf::from(format!(".temp_receiver_{}", ticket.hash().to_hex())));
    dbg!(temp_path.clone());

    if !temp_path.exists() {
        create_dir_all(temp_path.clone())?;
    }

    let store = FsStore::load(temp_path.clone()).await?;
    let hash = ticket.hash_and_format();
    let local = store.remote().local(hash).await?;

    if !local.is_complete() {
        let addr = ticket.node_addr();
        dbg!(addr.clone());
        let con = endpoint.connect(addr.clone(), iroh_blobs::ALPN).await?;
        dbg!(con.clone());
        let g = store.remote().execute_get(con, local.missing()).await?;
        println!(
            "Completado en: {}/{}b",
            g.elapsed.as_secs(),
            g.total_bytes_read()
        );
    }
    let collection = Collection::load(hash.hash, store.as_ref()).await?;
    let first_name = collection.iter().next();

    for (name, hash) in collection.iter() {
    let file_path = temp_path.clone().join(PathBuf::from(name));
    dbg!(file_path.clone());
    let mut stream_export = store
        .export_with_opts(ExportOptions {
            hash: *hash,
            mode: ExportMode::Copy,
            target: file_path,
        })
        .await?;
    dbg!(stream_export);
    }
    //TODO: Clone path into path
    let current_path = std::env::current_dir()?;
    let real_path = 
         if let Some(name_path) = path.clone() {
            if !name_path.exists() {create_dir_all(name_path.clone())?}
            let temp = name_path.join(PathBuf::from(first_name.unwrap().0.clone().split("/").next().unwrap())).clone();
            println!("exporting to {:?}", temp);
            let result = temp.into_os_string().into_string().unwrap();
            result
         }
        else{
        println!("exporting to {:?}", current_path);
        let temp = current_path.join(PathBuf::from(first_name.unwrap().0.clone().split("/").next().unwrap()));
            println!("exporting to {:?}", temp);
            let result = temp.into_os_string().into_string().unwrap();
            result
        };
    println!("REAL PATH: {}",real_path.clone());
    Repository::clone(temp_path.clone().join(PathBuf::from(first_name.unwrap().0.clone().split("/").next().unwrap())).to_str().unwrap(),real_path.clone())?;

    remove_dir_all(temp_path).await?;
    Ok(())
}
