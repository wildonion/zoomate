


use crate::{*, api::echo_service_server::EchoService};


/* this is our server */
#[derive(Clone, Debug, Default)]
pub struct EchoServer{}


/* ---------------------------------------------------------------------------------------
    node.proto is an actor which contains message structs and service handlers
    EchoService contains a method called echo which can be used to handle incoming 
    requests and send back the response, we're implementing its trait for the EchoServer 
    in here so we can handle the requests and send tonic response back to the caller
*/
#[tonic::async_trait]
impl EchoService for EchoServer{

    /* --------------------------------------------------------------
        echo is a method of the EchoService actor that can be called
        directly by the gRPC client
    */
    async fn echo(&self, request: TonicRequest<EchoRequest>) -> Result<TonicResponse<EchoResponse>, Status> {

        info!("Got a request {:?}", request);

        let resp = EchoResponse{
            message: format!("{}", request.into_inner().message),
        };

        Ok(TonicResponse::new(resp))

    }
}