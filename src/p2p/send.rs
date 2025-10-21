
use anyhow::{Ok, Result};
use iroh::{protocol::Router, Endpoint, SecretKey, Watcher};
use iroh_blobs::{
    api::
        blobs::{AddPathOptions, ImportMode}
    , format::collection::Collection, store::fs::FsStore, ticket::BlobTicket, BlobFormat, BlobsProtocol, Hash
};
use n0_future::{BufferedStreamExt, StreamExt};
use std::{
    ffi::OsString,
    path::PathBuf,
};
use tokio::fs::remove_dir_all;
use walkdir::WalkDir;


async fn send_path(path: PathBuf, blobs: BlobsProtocol,store: FsStore, router: Router) -> Result<()> {
    let archivos = WalkDir::new(path.canonicalize()?.clone())
        .into_iter()
        .filter_map(|f| {
            let f = f.ok()?;
            //dbg!(f.clone());



            if !f.file_type().is_file() {
                return None;
            }
            if f.path().iter().find(|x| *x == ".git").is_none() {
                //println!("NOT .git {:?}",f.file_name().to_str());
                return None;
            }
            println!(" .git {:?}",f.file_name().to_str());

            let name = f.clone().into_path().strip_prefix(path.canonicalize().unwrap().parent()?).unwrap().as_os_str().to_owned();
            let ph = f.clone().into_path();

            //println!("{:?} {:?}", name, ph);

            Some((name, ph))
        })
        .collect::<Vec<(OsString, PathBuf)>>();

    dbg!(archivos.clone());

    let res = n0_future::stream::iter(archivos)
        .map(|(f, p)| {
            let storage = blobs.store();
            let blob = storage.add_path_with_opts(AddPathOptions {
                path: p,
                format: BlobFormat::Raw,
                mode: ImportMode::TryReference,
            });
            async move { return (f.into_string().unwrap(), blob.await.unwrap().hash) }
        })
        .buffered_unordered(num_cpus::get())
        .collect::<Vec<(String, Hash)>>()
        .await;

    let coleccion = Collection::from_iter(res);
    let tag = coleccion.clone().store(&store).await?;
    let addr = router.endpoint().node_addr().initialized().await;
    let ticket = BlobTicket::new(addr, *tag.hash(), BlobFormat::HashSeq);
    println!("{:?}", coleccion);
    println!(" TICKET: {ticket}");

    Ok(())
}

pub async fn send(path: Option<PathBuf>, database: Option<PathBuf>) -> Result<()> {
    let mut rng = rand::rngs::OsRng;
    let key =  SecretKey::generate(&mut rng);
    let endpoint = Endpoint::builder().secret_key(key).discovery_n0().bind().await?;

    let database_path: PathBuf = match database {
        Some(ref a) => a.canonicalize()?,
        None => std::env::current_dir()?.join(PathBuf::from(format!(
            ".temp_sender_{:?}",
            if let Some(p) = path.clone() {p.canonicalize()?.file_name().unwrap().to_os_string()}else{OsString::from("default")}
        ))),
    };

    let store = FsStore::load(database_path.clone()).await?;

    let blobs: BlobsProtocol = BlobsProtocol::new(&store, endpoint.clone(), None);

    let router: Router = Router::builder(endpoint.clone())
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn();

    if path.is_some() {send_path(path.unwrap(),blobs,store,router.clone()).await?;}

    tokio::signal::ctrl_c().await;

    if database.is_none() {remove_dir_all(database_path).await?;}
    router.shutdown().await?;
    Ok(())
}

