use rocket::main;

use worker::web_workers::iqdb_client::IqdbClient;
use worker::worker_init;

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

    /*let stream_client = worker_init::init_iqdb();
    let mut sauce_jobs: FuturesUnordered<_> = (1..512)
        .map({
            let sc = &stream_client;
            move |i| async move {
                let sauce = sc.get_sauce(format!("Megumin {}", i)).await.unwrap();
                println!("The sauce of {} is {}", i, sauce);
            }
        }).collect();
    while let Some(_) = sauce_jobs.next().await {}*/

    /* for i in 1..512 {
        task::spawn(async {
            let req = format!("Megumin {}", i);
            let sauce = stream_client.get_sauce(req.to_string()).await.unwrap();
            println!("The sauce of {} is {}", req, sauce);
        });
    }; */

    /*
    stream::iter(1..512)
        .map(|num| format!("Megumin {}", num))
        .map(|req| (req, stream_client.get_sauce(req)))
        .for_each_concurrent(512, |(req, sauce)| async {
            println!("The sauce of {} is {}", req, sauce.await.unwrap());
        }).await;
    */

    let context = Context {
        client,
        config
    };

    server::launch_server(context).await?;

    Ok(())
}

pub struct Context {
    client: Box<dyn IqdbClient + Send + Sync>,
    config: ServerConfig,
}
