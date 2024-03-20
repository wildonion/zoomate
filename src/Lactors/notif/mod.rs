


use actix::prelude::*;
use redis_async::resp::FromResp;
use std::collections::HashMap;
use log::*;


#[derive(Clone)]
pub struct AppNotif;

#[derive(Clone, Message)]
#[rtype(result = "AppNotifs")]
pub struct GetAppNotifsMap;

#[derive(Clone)]
#[derive(MessageResponse)]
pub struct AppNotifs(pub Option<HashMap<i32, AppNotif>>);


// -------------------------------------
/* user notif subscriber actor worker */
// -------------------------------------
#[derive(Clone)]
pub struct NodeActionActor{
    pub node_notifs: Option<HashMap<i32, AppNotifs>>,
}


impl Actor for NodeActionActor{
    
    // actors run within a specific execution context Context<A>
    // the context object is available only during execution or ctx 
    // each actor has a separate execution context the execution 
    // context also controls the lifecycle of an actor.
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context){
        
        info!("NodeActionActor -> started subscription interval");

    }
    
}

impl NodeActionActor{

}