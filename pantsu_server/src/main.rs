use std::sync::Arc;
use tracing::{info, debug, Level};

use worker::fs::fs_service::FsService;
use worker::iqdb::iqdb_service::IqdbService;
use worker::worker_init;

use crate::common::result::Result;
use crate::config::ServerConfig;
use crate::log::setup_logger;

mod common;
mod config;
mod db;
mod fs;
mod image;
mod log;
mod server;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger(Level::DEBUG);
    let config = ServerConfig::load_config()?;
    println!("{:?}", config);
    debug!("{:?}", config);

    let iqdb_service = worker_init::init_iqdb();
    let sauce = iqdb_service.get_sauce("Megumin".to_string()).await?;
    info!("the sauce of {} is {}", "Megumin", sauce);

    let fs_service = worker_init::init_fs(config.clone());

    /*let stream_service = worker_init::init_iqdb();
    let mut sauce_jobs: FuturesUnordered<_> = (1..512)
        .map({
            let ss = &stream_service;
            move |i| async move {
                let sauce = ss.get_sauce(format!("Megumin {}", i)).await.unwrap();
                println!("The sauce of {} is {}", i, sauce);
            }
        }).collect();
    while let Some(_) = sauce_jobs.next().await {}*/

    /* for i in 1..512 {
        task::spawn(async {
            let req = format!("Megumin {}", i);
            let sauce = stream_service.get_sauce(req.to_string()).await.unwrap();
            println!("The sauce of {} is {}", req, sauce);
        });
    }; */

    /*
    stream::iter(1..512)
        .map(|num| format!("Megumin {}", num))
        .map(|req| (req, stream_service.get_sauce(req)))
        .for_each_concurrent(512, |(req, sauce)| async {
            println!("The sauce of {} is {}", req, sauce.await.unwrap());
        }).await;
    */

    let app_state = AppState {
        iqdb_service: Arc::new(iqdb_service),
        fs_service: Arc::new(fs_service),
        config
    };

    server::launch_server(app_state).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    iqdb_service: Arc<dyn IqdbService + Send + Sync>,
    fs_service: Arc<dyn FsService + Send + Sync>,
    config: ServerConfig,
}
