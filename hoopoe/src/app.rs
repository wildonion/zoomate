



use serde::{Serialize, Deserialize};
use s3req::*;
use multipartreq::*;

mod helpers;



#[actix_web::main]
async fn main() -> std::io::Result<()>{

    /*
        >_ running a tcp listener server, actix will use this to accept 
        incoming tcp based connections in its threadpool
    */
    dotenv::dotenv().expect(".env file must be in here!");
    let tcp_listener = std::net::TcpListener::bind(
    format!("{}:{}", 
            std::env::var("HOST").expect("⚠️ no host variable set"), 
            std::env::var("PANEL_PORT").expect("⚠️ no panel port variable set").parse::<u16>().unwrap()
    )).unwrap();

    let server = bootsteap!
    {
        /* SERVER CONFIGS */
        tcp_listener // bootstrapping the server with a custom tcp listener
    };

    server

}