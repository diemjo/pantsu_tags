use rocket::main;

use worker::web_workers::iqdb_service::IqdbService;
use worker::worker_init;

use crate::common::result::Result;
use crate::config::ServerConfig;

mod common;
mod config;
mod image;
mod server;
mod worker;

#[main]
async fn main() -> Result<()> {
    let config = ServerConfig::load_config()?;

    let iqdb_service = worker_init::init_iqdb();
    let sauce = iqdb_service.get_sauce("Megumin".to_string()).await?;
    println!("the sauce of {} is {}", "Megumin", sauce);

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

    let context = Context {
        iqdb_service,
        config
    };

    server::launch_server(context).await?;

    Ok(())
}

pub struct Context {
    iqdb_service: Box<dyn IqdbService + Send + Sync>,
    config: ServerConfig,
}
