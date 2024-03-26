use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use actix_telepathy::prelude::*;  // <-- Telepathy extension
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(RemoteMessage, Serialize, Deserialize)]  // <-- Telepathy extension
struct MyMessage {}

#[derive(RemoteActor)]  // <-- Telepathy extension
#[remote_messages(MyMessage)]  // <-- Telepathy extension
struct WorkLoad {
    state: usize
}

impl WorkLoad{

    // emit/fire/publish an event into a redis pubsub channel 
    // so other apis can subscribe to it
    pub async fn emit(&mut self){
        // redis and actix broker publication
        // ...
    }

    // redis/actixborker subscription process to subscribe to an specific channel 
    // contians emitted data from other apis publisher
    pub async fn subscribe(&mut self){
        // redis and actix broker subscription
        // ...
    }
    
}

impl Actor for WorkLoad {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register(ctx.address().recipient());  // <-- Telepathy extension
        self.subscribe_system_async::<ClusterLog>(ctx);  // <-- Telepathy extension
    }
}

impl Handler<MyMessage> for WorkLoad {
    type Result = ();

    fn handle(&mut self, msg: MyMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("RemoteMessage received!")
    }
}

impl Handler<ClusterLog> for WorkLoad {  // <-- Telepathy extension
    type Result = ();

    fn handle(&mut self, msg: ClusterLog, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClusterLog::NewMember(node) => {
                println!("New member joined the cluster.");
                let remote_addr = node.get_remote_addr(Self::ACTOR_ID.to_string());
                remote_addr.do_send(MyMessage {})
            },
            ClusterLog::MemberLeft(_ip_addr) => {
                println!("Member left the cluster.")
            }
        }
    }
}
impl ClusterListener for WorkLoad {}  // <-- Telepathy extension
