use rocket::futures::{stream, StreamExt};
use rocket::main;
use rocket::tokio::task;
use worker::worker_init;
use worker::web_workers::iqdb_client::IqdbClient;

use crate::common::result::Result;
use crate::config::ServerConfig;

mod common;
mod config;
mod server;
mod worker;

#[main]
async fn main() -> Result<()> {
    let config = ServerConfig::load_config()?;

    let client = worker_init::init_iqdb();
    let sauce = client.get_sauce("Megumin".to_string()).await?;
    println!("the sauce of {} is {}", "Megumin", sauce);
    let context = Context {
        client,
        config
    };

    /*let stream_client = worker_init::init_iqdb();
    for i in 1..512 {
        let idx = &i;
        let str_client = stream_client.clone();
        task::spawn(async move {
            let req = format!("Megumin {}", idx);
            let sauce = &str_client.get_sauce(req.to_string()).await.unwrap();
            println!("The sauce of {} is {}", req, sauce);
        });
    }*/


    /*
    stream::iter(1..512)
        .map(|num| format!("Megumin {}", num))
        .map(|req| (req, stream_client.get_sauce(req)))
        .for_each_concurrent(512, |(req, sauce)| async {
            println!("The sauce of {} is {}", req, sauce.await.unwrap());
        }).await;
    */

    server::launch_server(context).await?;

    Ok(())
}

pub struct Context {
    client: Box<dyn IqdbClient + Send + Sync>,
    config: ServerConfig,
}
