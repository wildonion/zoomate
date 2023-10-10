use std::sync::Arc;
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, RngCore};
use rand_chacha::{rand_core, ChaCha12Rng};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::{error, info};
use dotenv::dotenv;
use std::env;

mod redis4;
use crate::redis4::*;


/* 
    if we want to use Result<(), impl std::error::Error + Send + Sync + 'static>
    as the return type thus the error variable must be sepecified also the Error trait
    must be implemented for the error type (impl Error for ErrorType{}) since 
    we're implementing the Error trait for the error type in return type   
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{

    dotenv().expect("⚠️ .env file not found");
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //// usize is the minimum size in os which is 32 bits
    let redis_password = env::var("REDIS_PASSWORD").expect("⚠️ no redis password variable set");
    let redis_host = std::env::var("REDIS_HOST").expect("⚠️ no redis host variable set");
    let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_host);
    let redis_client = redis::Client::open(redis_conn_url.as_str()).unwrap();
    let (redis_pubsub_msg_sender, mut redis_pubsubs_msg_receiver) = tokio::sync::mpsc::channel::<String>(io_buffer_size);
    let mut redis_conn = redis_client.get_connection().unwrap();


    /* 
        for each async and heavy tasks we must use 
        tokio::spawn(async move{}) channel to avoid 
        blocking issues, stuck and halting situations 
    */


    tokio::spawn(async move{

        while let Some(data) = redis_pubsubs_msg_receiver.recv().await{

            // receiving data from the redis pubsub mpsc sender
            // ...

        }

    });


    tokio::spawn(async move{

        /* start an async and concurrent server to handle socket packets from clients concurrently */ 
        start_server(|req, res| async move{
            Ok(
                Response{}
            )
        }, redis_pubsub_msg_sender.clone(), redis_client.clone()).await
    
    });



    Ok(())


}