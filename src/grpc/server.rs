


use crate::{*, node::node_service_server::NodeService};
use tonic::transport::Server as TonicServer;

/* > ------------------------------------------------------------------
   | -> each rpc ds is like an actix actor which contains:
   |    * inner message handlers to communicate with different parts of the app's actors
   |    * tcp based stream handlers and listeners to stream over incoming connections 
   |      to map packet bytes like Capnp, Protobuf, serde_json, Multipart, BSON and 
   |      Payload into desired data struct
   |    * inner concurrent task handlers for sending/receiving message, handling
   |      async tasks from outside of the app using tokio::spawn,mpsc,mailbox,mutex,
   |      rwlock,select,time 
   | -> two actors in two apps communicate through streaming and pubsub channels using rcp http2 and redis
   | -> two actors in an app communicate through streaming and pubsub channels using mpsc and redis 
   |
*/
#[derive(Clone, Debug, Default)]
pub struct NodeServer{}

impl NodeServer{

    pub async fn start(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{

        info!("➔ 🚀 gRPC server has launched from [{}] at {}", 
            addr, chrono::Local::now().naive_local());

        let node = NodeServer::default();
        info!("gRPC Server listening on {}", addr);
    
        // node webhook signature
        let node_instance = Node::default();
        let (pubkey, prvkey) = node_instance.generate_ed25519_webhook_keypair();
        println!("ed25519 pubkey: {}", pubkey);
        println!("ed25519 prvkey: {}", prvkey);
        
        TonicServer::builder()
            /* creating a new server service actor from the EchoServer structure which is our rpc server */
            .add_service(NodeServiceServer::new(node))
            .serve(addr)
            .await
            .unwrap();
        
        Ok(())
        
    }

    fn restart() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{

        Ok(())

    }

}


/* -----------------------------------------------------------------------------------------------
    every actor like actix actors must have some message structs to send pre defined message
    to other actors or send message from different parts of the app to the actor also it must
    contains a handler to handle the incoming messages or streams and respond the caller with
    appropriate message, node.proto is an actor which contains message structs and service 
    handlers NodeService handler contains a method called verify which can be used to handle 
    incoming requests and send a response back to the caller, we're implementing the NodeService 
    handlder trait for the NodeServer struct in here which allows us to handle and accept the 
    requests and send tonic response directly back to the caller of the verify method, for every
    service handler in proto file we have to implement the trait in here for the server struct
    so client can call the methods directly.

    each service in protobuf is a trait in rust which allows us to implement it for any server 
    struct and overwrite its handler methods to accept rpc request in form of single structure
    or streaming and sequencing of data structures.
*/
#[tonic::async_trait]
impl NodeService for NodeServer{

    /* --------------------------------------------------------------
        echo is a method of the NodeService actor that can be called
        directly by the gRPC client
    */
    async fn echo(&self, request: TonicRequest<NodeRequest>) -> Result<TonicResponse<NodeResponse>, Status> {

        info!("got an gRPC request at time {:?} | {:?}", 
            chrono::Local::now().naive_local(), request);
        
        let request_parts = request.into_parts();
        let node_rpc_request_body = request_parts.2;
        let metadata = request_parts.0;
        let get_headeres = metadata.get("authorization");
        

        match get_headeres{

            Some(metadata_value) => {

                let jwt = format!("Bearer {}", metadata_value.to_str().unwrap());
                let node_resp = NodeResponse::default();
                Ok(TonicResponse::new(node_resp))


            },
            None => {

                error!("found no jwt in metadata {:?}", chrono::Local::now().naive_local());
                let node_resp = NodeResponse::default();
                Err(Status::unauthenticated("invalid token"))

            }
        }

    }
}