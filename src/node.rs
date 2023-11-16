use std::net::SocketAddr;
use std::sync::Arc;
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, RngCore};
use rand_chacha::{rand_core, ChaCha12Rng};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::sync::broadcast;
use tokio::sync::futures;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use zoomate::Node;
use std::collections::HashMap;
use log::{error, info};
use dotenv::dotenv;
use std::env;
use sha2::{Sha256, Digest};
use tonic::{transport::Server, Request as TonicRequest, Response as TonicResponse, Status};
use crate::api::echo_service_server::EchoServiceServer;
use crate::grpc::server::EchoServer;
use crate::redis4::*;
use api::{EchoRequest, EchoResponse, echo_service_client, echo_service_server};
use ::clap::{Parser};
use zoomate::api;


mod redis4;

mod grpc;

mod raptor;
use crate::raptor::*;

mod bpf;
use crate::bpf::*;



/* ---------------------------------------------------------
    loading the compiled proto file into rust code in here 
    contains traits and data structures to use them in here 
    to create rpc server and client
*/
pub mod api{
    tonic::include_proto!("api");
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


    /* 
        if we need a data to be shared with other scopes which is inside of
        spawned task we have to use mpsc like channel to move it between other 
        scopes, with tokio::spawn() we can run async tasks in another threadpool
        using green threads other than main threads which prevent other codes 
        from being halted in their execution while we're running the async task 
        also std::thread by default handles the multi-processing manner when the
        task is heavy enough so it can switch to another navtive core.
    */
    tokio::spawn(async move{
        
        // hadead rate limiter 
        let res = api().await;
        println!("hadead res {:?}", res);

    });


    /* ------------------------------ */
    /*      start grpc server         */
    /* ------------------------------ */
    // cargo run --bin zoomate -- --server 0.0.0.0 --port 50051
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse::<SocketAddr>().unwrap();
    let node = EchoServer::default();
    info!("gRPC Server listening on {}", addr);

    // node webhook signature
    let node_instance = Node::default();
    let (pubkey, prvkey) = node_instance.generate_ed25519_webhook_keypair();
    println!("ed25519 pubkey: {}", pubkey);
    println!("ed25519 prvkey: {}", prvkey);
    
    Server::builder()
        .add_service(EchoServiceServer::new(node))
        .serve(addr)
        .await
        .unwrap();

    

    Ok(())


}