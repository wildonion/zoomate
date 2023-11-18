


use crate::{*, node::node_service_server::NodeService};


/* this is our server */
#[derive(Clone, Debug, Default)]
pub struct NodeServer{}


/* -----------------------------------------------------------------------------------------
    every actor like actix actors must have some message structs to send pre defined message
    to other actors or send message from different parts of the app to the actor also it must
    contains a handler to handle the incoming messages or streams and respond the caller with
    appropriate message, node.proto is an actor which contains message structs and service 
    handlers NodeService handler contains a method called echo which can be used to handle 
    incoming requests and send a response back to the caller, we're implementing the NodeService 
    handlder trait for the NodeServer struct in here which allows us to handle and accept the 
    requests and send tonic response directly back to the caller of the echo method, for every
    service handler in proto file we have to implement the trait in here for the server struct
    so client can call the methods directly.
*/
#[tonic::async_trait]
impl NodeService for NodeServer{

    /* --------------------------------------------------------------
        echo is a method of the NodeService actor that can be called
        directly by the gRPC client
    */
    async fn echo(&self, request: TonicRequest<NodeRequest>) -> Result<TonicResponse<NodeResponse>, Status> {

        info!("Got a request {:?}", request);

        let resp = NodeResponse{
            message: format!("{}", request.into_inner().message),
        };

        Ok(TonicResponse::new(resp))

    }
}