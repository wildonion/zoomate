



/*  > -----------------------------------------------------------------------
    | deploy actor is used to deploy the app by sending command to this actor
    | -----------------------------------------------------------------------
    | contains: message structures and their handlers
    |
*/

use crate::*;
use actix::prelude::*;
use s3req::Storage;
use redis_async::resp::FromResp;
use actix::*;
use log::*;

pub struct DeployAgentActor{
    pub port: u16,
    pub path: std::path::PathBuf // use to store the path of deploy service script
}

impl Actor for DeployAgentActor{

    type Context = Context<Self>; // ctx contains the whole actor instance and its lifecycle execution

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("DeployAgentActor is started");
    }

}


impl DeployAgentActor{

    pub fn new(port: u16, path: std::path::PathBuf) -> Self{
        Self { port, path}
    }

    // automatic deploy: run redeoploy.sh or rebuildpanel.sh scripes
    // github actions and yml file
    pub fn cid_cd(){
        
    }
}