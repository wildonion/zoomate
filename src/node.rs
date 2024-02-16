use std::net::SocketAddr;
use std::sync::Arc;
use actix::Actor;
use constants::SECURECELLCONFIG_TCPWALLET;
use env_logger::Env;
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
use utils::{ZoomateRequest, ZoomateResponse};


mod redis4;

mod tcpserver;

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
#[command(about = "zoomate grpc server config", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}


/*  
    https://github.com/actix/actix-web/blob/master/actix-web/MIGRATION-4.0.md#actix_webmain-and-tokiomain
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
    -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    {

    dotenv().expect("⚠️ .env file not found");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //// usize is the minimum size in os which is 32 bits
    let redis_password = env::var("REDIS_PASSWORD").expect("⚠️ no redis password variable set");
    let redis_host = std::env::var("REDIS_HOST").expect("⚠️ no redis host variable set");
    let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_host);
    let redis_client = redis::Client::open(redis_conn_url.as_str()).unwrap();
    let (redis_pubsub_msg_sender, mut redis_pubsubs_msg_receiver) = tokio::sync::mpsc::channel::<String>(io_buffer_size);
    let mut redis_conn = redis_client.get_connection().unwrap();


    /*  
         -------------------------------------------------------------------------------------------
        |                    NOTE ON CODE ORDER EXECUTION OF ASYNC METHODS
        |-------------------------------------------------------------------------------------------
        | 
        | in rust the order of execution is not async by default but rather it's thread safe 
        | and without having race conditions due to its rules of mutable and immutable pointers 
        | of types although if there might be async methods but it's not expected that they must 
        | be executed asyncly, the early one gets executed first and then the second one goes, 
        | an example of that would be calling async_method_one() method with async operations 
        | inside, and other_async_method_two() method, both of them are async however, but the 
        | code waits till all the async operations inside the first one get executed then run the 
        | second one, this gets criticized if we have some delay and sleep methods inside the first 
        | one which gets us into trouble with the whole process of code order execution if we don't 
        | want to have disruption in their execution, though in some cases it's needed to have this 
        | logic but in here it would be a bad idea, the solution to this is running both of them 
        | asyncly in their own seprate threadpool which can be done by putting each of them inside 
        | tokio::spawn() in this case there would be no disruption in their order execution at all 
        | and we'd have a fully async execution of methods in the background.
        | to catch any result data inside the tokio::spawn() we would have to use mpsc channel to
        | send the data to the channel inside the tokio::spawn() and receive it outside of tokio
        | scope and do the rest of the logics with that.
        |
        | conclusion: 
        | use tokio::spawn() to execute any async task in the background without having
        | any disruption in other order execution of async methods.
        |
    */


    //---------------------------------------------------------------
    //---- start redis4 server with tokio::spawn() in the background
    //---------------------------------------------------------------
    // spawn the method in the background asyncly and concurrently
    // without having any disruption in order execution with other
    // aync methods 
    tokio::spawn(async move{
        start_server(|req, res| async move{
            Ok(
                Response{}
            )
        }, redis_pubsub_msg_sender.clone(), redis_client.clone()).await
    });


    /* ------------------------------ */
    /*    start tcp listener actor    */
    /* ------------------------------ */
    /* 
        first we must get the shared tcp ed25519 secure cell config 
        and wallet to stablish secured communication
        
        can't start streaming over a tokio tcp listener using actix actor
        because of different version of multiple tokio runtimes

        spawn the method in the background asyncly and concurrently
        without having any disruption in order execution with other
        aync methods
    */
    tokio::spawn(async move{
        let (mut secure_cell, wallet) = SECURECELLCONFIG_TCPWALLET.to_owned(); //---- this must be shared between clients and server
        let tcp_addr = format!(
                "{}:{}",
                std::env::var("TCP_HOST").unwrap(),
                std::env::var("TCP_PORT").unwrap()
            );
        let listener_actor = tcpserver::TcpListenerActor::new(wallet, secure_cell, &tcp_addr);
        // --------------------
        // don't start actor inside tokio::spawn cause they must be executed from the context of actix runtime itself
        // ERROR: `spawn_local` called from outside of a `task::LocalSet`
        // SOLUTION: use #[actix_web::main] on top of main function since the actors must be executed from the context of actix_web runtime itself and outside of the tokio::spawn
        // let tcp_listener_actor_address = listener_actor.start(); //--- this will be run but shows the above error
        // --------------------
        listener_actor.start_streaming().await;
    });
    

    /* ------------------------------------------- */
    // NODEJS LIKE ASYNC METHOD ORDER OF EXECUTION
    /* ------------------------------------------- */
    // execution of async methods are not async we should put them in tokio::spawn
    tokio::spawn(async move{
        hello().await;
    });

    async fn request(){
        for i in 0..10{
            info!("i => {:?}", i);
        }
    }

    async fn hello(){
        info!("hello");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        info!("waking up after 3 seconds in hello method");
    } 

    tokio::spawn(async move{
        request().await;
    });


    /* ------------------------------ */
    /*     start grpc server actor    */
    /* ------------------------------ */
    // spawn the method in the background asyncly and concurrently
    // without having any disruption in order execution with other
    // aync methods
    tokio::spawn(async move{
        let cli = ServerCli::parse();
        let addr = format!("{}:{}", cli.server, cli.port).parse::<SocketAddr>().unwrap();
        server::NodeServer::start(addr).await;
    });
    

    // async sleeping in the current thread without having 
    // disruption in other code order execution
    tokio::spawn(async move{
        info!("sleeping in the background asnycly");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        info!("waking up after 2 secs");
    });


    //----------------------------------------------------------------------------------
    //---- file encryption using ed25519 wallet with aes256 themis secure cell signing
    //----------------------------------------------------------------------------------
    // spawn the method in the background asyncly and concurrently
    // without having any disruption in order execution with other
    // aync methods
    tokio::spawn(async move{
        let mut encrypted = cry::wannacry::encrypt_file("secret.txt").await;
        let decrypted = cry::wannacry::decrypt_file("secret.txt.dec", &mut encrypted.1).await;
        // constants::serding().await; // .await pins the future into the ram before polling it
    });
    
    /* 
        we should make the app to be ran constantly like a real server, so we can monitor the
        logics inside any tokio::spawn() or other threads which have been executed concurrently
        and asyncly in the background, otherwise we would use some mpsc channel to send any 
        computational result inside of those threads into the channel so we can receive it 
        outside of their scopes while the app is running
    */
    loop{} //--- halt the code in here and making it to be ran constantly, so sockets can be processed

}