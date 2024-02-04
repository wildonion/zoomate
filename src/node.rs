use std::net::SocketAddr;
use std::sync::Arc;
use actix::Actor;
use grpc::server;
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, RngCore};
use rand_chacha::{rand_core, ChaCha12Rng};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::sync::broadcast;
use tokio::sync::futures;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::{error, info};
use dotenv::dotenv;
use std::env;
use sha2::{Sha256, Digest};
use tonic::{transport::Server, Request as TonicRequest, Response as TonicResponse, Status};
use crate::grpc::server::NodeServer;
use crate::redis4::*;
use node::{NodeRequest, NodeResponse, node_service_client::NodeServiceClient, node_service_server::NodeServiceServer};
use ::clap::{Parser};
use utils::{api, ZoomateRequest, ZoomateResponse};


mod redis4;

mod tcpactor;

mod constants;

mod grpc;

mod raptor;
use crate::raptor::*;

mod bpf;
use crate::bpf::*;

mod extractor;

mod cry;
use crate::cry::*;



/* ---------------------------------------------------------
    loading the compiled proto file into rust code in here 
    contains traits and data structures to use them in here 
    to create rpc server and client
*/
pub mod node{
    tonic::include_proto!("node");
}


#[derive(Parser)]
#[command(author, version)]
#[command(about = "zoomate grpc server", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}


/*  
    instead of addin' #[tokio::main] macro on top of the main method which will execute
    the whole main method and all of its functions inside of it in a threadpool context 
    we can remove the macro to have a none async main method but still in order to run 
    async methods inside of it we can't just call them and put .await on them cause rust 
    is not a async lang by nature and in order to solve async method we must be inside an 
    async context thus we need an async env to do so which can be solved by sending the 
    async method or job into the tokio::spawn() which is a async job or task handler in 
    it's threadpool context behind the scene without having deadlocks and race conditions
*/
#[tokio::main]
async fn main() 
// fn main() 
    
    /* 
        if we want to use Result<(), impl std::error::Error + Send + Sync + 'static>
        as the return type thus the error variable must be sepecified also the Error trait
        must be implemented for the error type (impl Error for ErrorType{}) since 
        we're implementing the Error trait for the error type in return type   
    */
    -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{

    dotenv().expect("⚠️ .env file not found");
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //// usize is the minimum size in os which is 32 bits
    let redis_password = env::var("REDIS_PASSWORD").expect("⚠️ no redis password variable set");
    let redis_host = std::env::var("REDIS_HOST").expect("⚠️ no redis host variable set");
    let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_host);
    let redis_client = redis::Client::open(redis_conn_url.as_str()).unwrap();
    let (redis_pubsub_msg_sender, mut redis_pubsubs_msg_receiver) = tokio::sync::mpsc::channel::<String>(io_buffer_size);
    let mut redis_conn = redis_client.get_connection().unwrap();


    /* 
        for handling and streaming over each coming async and heavy tasks we must use 
        tokio::spawn(async move{}) to avoid blocking issues, stuck and halting situations 
    */
    tokio::spawn(async move{

        while let Some(data) = redis_pubsubs_msg_receiver.recv().await{

            // receiving data from the redis pubsub mpsc sender
            // ...

        }

    });


    /* start redis4 server in tokio::spawn() threads */
    tokio::spawn(async move{

        /* start an async and concurrent server to handle socket packets from clients concurrently */ 
        start_server(|req, res| async move{
            Ok(
                Response{}
            )
        }, redis_pubsub_msg_sender.clone(), redis_client.clone()).await
    
    });



    //----------------------------------------------------------------------------------
    //---- file encryption using ed25519 wallet with aes256 themis secure cell signing
    //----------------------------------------------------------------------------------
    let mut encrypted = utils::encrypt_file("secret.txt").await;
    let decrypted = utils::decrypt_file("secret.txt.dec", &mut encrypted.1).await;


    /* ------------------------------ */
    /*    start tcp listener actor    */
    /* ------------------------------ */
    tcpactor::TcpListenerActor::new(&format!("0.0.0.0:2247")).start();


    /* ------------------------------ */
    /*     start grpc server actor    */
    /* ------------------------------ */
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse::<SocketAddr>().unwrap();
    server::NodeServer::start(addr).await

}