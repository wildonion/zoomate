


use crate::*;


pub async fn send_grpc_request(addr: &str) -> Result<(), Box<dyn std::error::Error>>{

    /* creating a new client service actor to connect to node addr */
    let mut client = NodeServiceClient::connect(addr.to_string()).await.unwrap();

    /* creating a new tonic request to send message to server actor */
    let request = TonicRequest::new(NodeRequest{
        message: "sending request to node...".to_string(),
    });

    /* -------------------------------------------------------------------------------------------
        sending request from client actor to server actor using EchoService handler defined 
        inside the node.proto by calling the echo method of the NodeServiceClient struct directly 
        which accepts a request object as param and returns a response object, the echo method 
        of the EchoService handler has been over written in server.rs for EchoServer struct 
        which allows us to handle the incoming requests in rust code and send them a response 
        object back to the caller or client
    */
    let response = client.echo(request).await.unwrap();
    info!("Got a response={:?}", response.into_inner().message);
    
    
    Ok(())
    
}