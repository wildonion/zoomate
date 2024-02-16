

use std::collections::HashMap;
use actix::{Actor, Handler, Message, StreamHandler};
use actix_web::{post, HttpRequest, HttpResponse};
use actix_web::web::Payload;
use aes256ctr_poly1305aes::aead::Buffer;
use chacha20::cipher::typenum::Len;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use redis_async::{resp::FromResp, client::ConnectionBuilder};
mod dp;
use dp::*;
mod acter;
use acter::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use once_cell::sync::Lazy;
use base64::{engine::general_purpose, Engine as _};
use wallexerr::misc::*;
use sha3::{Digest, Keccak256, Keccak256Core};
use ring::rand as ring_rand;
mod constants;
use constants::*;
mod misc;
use misc::*; // load all macros
mod cry;



/* 
    the ok arm of return type is an HttpResponse object which can be 
    parsed in any server or client application and can be sent through
    the tcp socket to either ckient or seerver
*/
#[post("/api")]
pub async fn api(req: HttpRequest, mut stream: Payload) 
    -> Result<actix_web::HttpResponse, actix_web::Error>{

    /* we have to fill a buffer on server with incoming bytes by streaming over `stream` object */
    let mut bytes = vec![];
    while let Some(item) = stream.next().await {
        bytes.extend_from_slice(&item?);
    }

    tokio::spawn(async move{

        // other api logics
        // ...
    
    });
   

    return Ok(
        HttpResponse::NotAcceptable().json("rate limited")
    );


}


/* 
    Send and Sync can only be implement for a type that is inside the current crate 
    thus can't be implemented for actix_web::HttpResponse
*/
unsafe impl Send for ZoomateResponse{}
unsafe impl Sync for ZoomateResponse{}

#[derive(Serialize, Deserialize, Copy, Clone, Debug , Default)]
pub struct ZoomateRequest; //// it can be Option<Vec<actix_web::HttpResponse>> which all the incoming actix http requests to this node that must be handled

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ZoomateResponse{
    pub data: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Weight{
    pub n: u16,
    pub requests: ZoomateRequest,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Streamer<'s>{
    pub body: &'s [u8]
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Node{ //// this contains server info 
    pub dns: String,
    pub peer_id: String, 
    pub cost_per_api_call: u128, //// this is based on the load of the weights
    pub init_at: i64,
    pub weights: Option<Vec<Weight>>, //// load of requests
    pub hash: String,
    pub nodes: Vec<Node>,
    pub req: ZoomateRequest,
    pub res: ZoomateResponse
}

impl Node{

    pub async fn get_agents<'lifetime, G: ?Sized>() -> Self 
        where G: Send + Sync + 'lifetime + Clone + std::fmt::Debug{

            Node::default()
    }

    pub async fn get_node_address(&mut self){

        /*
            by default every heap data will be moved into new type 
            when we're putting them inside another type thus we must 
            either clone or borrow them using & or as_ref() 
        */
        let nodes: &Vec<Node> = self.nodes.as_ref();
    }

    pub async fn hash_node(&mut self){

        let stringified_obj = serde_json::to_string_pretty(&self).unwrap();
        let hash = Wallet::generate_keccak256_hash_from(&stringified_obj);
        self.hash = hex::encode(&hash);
    }

    pub async fn connect_to_peer(addr: &str){

        let queue = Queue{
            vector: vec![
                Queue::default()
            ]
        };
        let get_queue = (
            |mut old_vec: Vec<Queue>|{
                let vec = Queue{
                    vector: vec![
                        Queue::default()
                    ]
                };
                old_vec.push(vec);
                old_vec
            }
        )(queue.vector);

        #[derive(Default)]
        struct Queue{
            pub vector: Vec<Queue>
        }

        let tcp_stream = tokio::net::TcpStream::connect(addr).await;
        if let Ok(mut streamer) = tcp_stream{

            let f = tokio::fs::File::open("readmeasync.txt").await;
            if let Err(why) = f.as_ref(){
                println!("can't create file cause: {}", why.to_string());
            }

            let mut buffer: Vec<u8> = vec![];
            let buf_bytes = buffer.as_mut_slice();
            f.unwrap().read(buf_bytes).await;

            /* write the fulfilled buffer from file bytes into the streamer */
            streamer.write_all(buf_bytes).await;
        }

    }

    pub async fn proof_of_chain(chain_addresses: Vec<String>, chain_addresses_from_other_nodes: Vec<String>){

        /*      ------ good for whilelist ------
            having two different mutable pointer to instances are not allowed in a single scope
            based on this we're ok to call calculate_root_hash() method which takes a mutable pointer 
            of the struct instance method two times on the same instance 
        */
        let mut merkle_tree_wl = constants::MerkleNode::new();
        let old_merkle_hash = merkle_tree_wl.calculate_root_hash(chain_addresses_from_other_nodes);  
        let merkle_hash = merkle_tree_wl.calculate_root_hash(chain_addresses);
        if old_merkle_hash == merkle_hash{
            
            // fork allowed
            // ...

        } else{
            
            // fork not allowed unknown address in the passed in addresses
            // ...
        }
    }

    pub async fn verify_update_signature(data: &str, signature: &str, pubkey: &str){

        // data: raw stringified data
        // signature
        // pubkey

        let data_hash = Wallet::generate_keccak256_hash_from(data);

    }

    pub async fn verify_api_signature(data: &str, signature: &str, pubkey: &str) -> RuntimeCode{

        // data: raw stringified data
        // signature
        // pubkey

        let data_hash = Wallet::generate_keccak256_hash_from(data);

        let is_verified = true;
        if is_verified{
            RuntimeCode::Ok(1)
        } else{
            RuntimeCode::Err(1)
        }
    }

    pub async fn encoder(data: impl Serialize){

        /* 
            note that the data param must implement the Serialize trait 
            so we can encode it to bytes
        */

    }

    pub async fn decoder(data: impl Deserialize<'_>){

        /* 
            note that the data param must implement the Deserialize trait 
            so we can decode it to the actual type
        */

    }

    pub async fn broadcast_to_other_nodes(node_obj: &str){

        /* hash of keccak256 of node_obj to send in network */
        let node_obj_hash = web3::signing::keccak256(node_obj.as_bytes());
        let node_obj_hash_hex = hex::encode(node_obj_hash);

        println!("boradcast node obj keccak256 hash : {:?}", node_obj_hash_hex);
    }

    pub fn generate_ed25519_webhook_keypair(&self) -> (String, String){

        let mut data = DataBucket{
            value: serde_json::to_string_pretty(&self).unwrap(), /* json stringifing the self */ 
            signature: "".to_string(),
            signed_at: 0,
        };
        let stringify_data = serde_json::to_string_pretty(&data).unwrap();

        /* wallet operations */

        let contract = Contract::new_with_ed25519("0xDE6D7045Df57346Ec6A70DfE1518Ae7Fe61113f4");
        
        /* 
            will be saved in a folder called wallexerr-keys inside the root of this project 
            since the wallexerr lib.rs is loaded in this project thus the root of wallexerr
            is the root of this project
        */
        Wallet::save_to_json(&contract.wallet, "ed25519").unwrap();
        
        let signature_hex = Wallet::ed25519_sign(
            stringify_data.clone().as_str(), 
            contract.wallet.ed25519_secret_key.as_ref().unwrap().as_str());
        
        let hash_of_data = Wallet::generate_keccak256_hash_from(&stringify_data);
        let verify_res = Wallet::verify_ed25519_signature(
            signature_hex.clone().unwrap().as_str(), hash_of_data.as_slice(),
            contract.wallet.ed25519_public_key.as_ref().unwrap().as_str());

        let keypair = Wallet::retrieve_ed25519_keypair(
            /* 
                unwrap() takes the ownership of the type hence we must borrow 
                the type before calling it using as_ref() 
            */
            contract.wallet.ed25519_secret_key.as_ref().unwrap().as_str()
        );

        
        match verify_res{
            Ok(is_verified) => {
                
                /* fill the signature and signed_at fields if the signature was valid */
                data.signature = signature_hex.unwrap();
                data.signed_at = chrono::Local::now().timestamp_nanos_opt().unwrap();
                
                (
                    contract.wallet.ed25519_public_key.unwrap(), 
                    contract.wallet.ed25519_secret_key.unwrap()
                )

            },
            Err(e) => (String::from(""), String::from(""))
        }
    
    }


}


// vpn streamer actor uses this signing method to ecnrypt data
pub fn ed25519_with_aes_signing(data: &str, mut wallet: Wallet) -> String{
    let aes256_signature = cry::eddsa_with_symmetric_signing::ed25519_aes256_signing(data, wallet.clone());
    let secure_cell_signature = cry::eddsa_with_symmetric_signing::ed25519_secure_cell_signing(data, wallet.clone());
    let keccak256_signature = cry::eddsa_with_keccak256_signing::ed25519_keccak256_signing(data, wallet.clone());

    secure_cell_signature
}

/* ----------------------------------------------------------------------- */
/* --------- actix ws stream and message handler for Node struct --------- */
/* ----------------------------------------------------------------------- */
/* 
    realtime networking and event driven coding using redispubsub, tokio stuffs 
    and actix web/ws stream/event handler like aggregate streaming of resp.boy 
    bytes into a buffer then decode the fulfilled into struct
*/
#[derive(Message)]
#[rtype(result = "()")]
pub struct NodeMsg(pub String);

impl Actor for Node{
    type Context = actix_web_actors::ws::WebsocketContext<Node>;
}

impl Handler<NodeMsg> for Node {
   
    type Result = ();

    fn handle(&mut self, msg: NodeMsg, ctx: &mut Self::Context){
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>> for Node{
    
    fn handle(&mut self, item: Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>, ctx: &mut Self::Context) {
        
        todo!()
    
    }
}

// custom stream handler for an actor
trait CustomStreamHandler{
    type Context;
    fn handle(&self, ctx: &mut Self::Context) -> ();
}
enum EnumTor{}
struct ActorStruct{
    pub enumtor: EnumTor
} 
impl ActorStruct{
    pub fn start(&self){
        
    }
}
impl CustomStreamHandler for ActorStruct{

    type Context = ActorStruct;
    fn handle(&self, ctx: &mut Self::Context) -> (){
        self.start();
    }
}
/* ----------------------------------------------------------------------- */
/* ----------------------------------------------------------------------- */
/* ----------------------------------------------------------------------- */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Container{
    pub id: String,
    pub balancer: Balancer,
    pub nodes: Vec<Node>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Balancer{
    RoundRobin,
    LeastConnection,
    WeightedLeastConnection,
    WeightedResponseTime,
    ResourceBased,
    WeightedRoundRobin,
    IpHash,
}


//// TODO - 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod{ //// a pod is a load balancer which can have one or more containers 
    pub id: String,
    pub containers: Vec<Container>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ResponseObject{
    data: String,
}

// a dns over httsp structures
pub struct Dns{
    pub queries: Vec<DnsRequest>,
    pub id: String
}

pub struct DnsRequest{
    pub http_req: HttpRequest 
}

pub async fn set_response<'lifetime, G, T: Send + Sync + 'static + FnMut() -> G>
    /* since T is a FnMut closure, the cls param must be defined mutablly */
    (mut cls: T){

    {
        let data = constants::IN_MEMORY_DB.clone();
        let mut map = data.lock().await;
        (*map).insert(100, "key".to_string());
    }

    /* T is a closure which returns G and can be shared between threads safely */
    let callback = cls();

    let mut res = ZOOMATE_RESPONE_STORAGE.lock().await;
    let new_data = vec![1,2,4];
    let strigified_data = serde_json::to_string_pretty(&new_data).unwrap();
    
    /* overriding the response object globally without having deadlocks and race conditions */
    (*res).data = strigified_data;

    // tokio crontab scheduler
    let mut time = tokio::time::interval(tokio::time::Duration::from_secs(5));
    loop{
        time.tick().await;
        println!("tick 1 sec");
    }

}


pub async fn agent_simulation<N>() where N: Send + Sync + 'static + Clone{

	let new_rt = tokio::runtime::Builder::new_multi_thread();

    type Cls<G> = Box<dyn std::future::Future<Output=G> + Send + Sync + 'static>;
    fn execute<V>(cls: Cls<V>) where V: Send + Sync + 'static + Clone{} 
    let method: fn(Cls<N>) -> () = execute;
    fn executeMe<N>(func: fn(Cls<N>) -> ()) -> Result<(), ()> 
    {
        
        Ok(())
    }
    executeMe(method);
    
	
    #[derive(Clone)]
	struct BuildQueue{
		pub agent_id: String,
	}
	#[derive(Clone)]
	struct Pipeline{
		pub pid: String, // keccak256 bits hash of the whole data and system usage
	}
    #[derive(Clone)]
    /* trait objects are heap data and must be beind pointer, eiter Box<dyn or &dyn */
    struct JobTor<'j>(pub &'j dyn FnMut() -> ());
    struct JobTorBox<'j>(pub Box<&'j dyn FnMut() -> ()>);

	/*
	    interior mutablity, we can mutate the field at runtime
	    it's usefull for mutating the content data inside an account
	*/
	#[derive(Clone)]
	struct NodeData<'v>(pub std::sync::Arc<tokio::sync::Mutex<&'v [u8]>>); 
	/* single thread version of NodeData */
	// struct NodeData<'v>(pub std::rc::Rc<std::cell::RefCell<&'v [u8]>>); 
	type Job<'validlifetime> = NodeData<'validlifetime>;
	#[derive(Clone)]
	struct Task{ /* job can be send between threads safely */
	    pub task: NodeData<'static>
	}
	#[derive(Clone)]
	struct Agent<'j> where Task: Send + Sync + 'static{
	    pub aid: String, // keccak256 bist hash of the whole data
	    pub jobs: &'j [Task],
	    pub pipeline: Pipeline
	}
	impl<'j> Agent<'j>{

	    async fn execute(&'static mut self, new_commit_data: &[u8]) -> Result<(), ()>{
            let jobs = self.jobs.clone();
            /* 
                accessing element inside array must be done behind pointer cause by accessing
                the element we're creating an slice which must be behind pointer cause they have
                no fixed size at compile time
            */
            let t = &jobs[0].task.0;
            tokio::spawn(async move{
                let data = *t.lock().await;
                // mutate data in here with new_commit_data
                // ...
            });
        
            Ok(())
	    }

	    async fn subscribe_to_new_commit(commit_id: &'static str) -> Result<(), ()>{
		    
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
                
            let mut redis_conn_builder = ConnectionBuilder::new("redis_host", 6379 as u16).unwrap();
            redis_conn_builder.password("redis_password");
            let async_redis_pubsub_conn = std::sync::Arc::new(redis_conn_builder.pubsub_connect().await.unwrap());

            tokio::spawn(async move{
			    
			    loop{

                    /* tick every 15 seconds */
                    interval.tick().await;
			
				    // setup async redis subscription process to subscribe to 
				    // ...
                    
				    let get_stream_messages = async_redis_pubsub_conn
                        .subscribe(commit_id)
                        .await;
				    
				    let Ok(mut get_stream_messages) = get_stream_messages else{
					    
					    return Err::<(), ()>(());
			
				    };
				
				    /* 
                        iterating through the msg future object streams as they're 
                        coming to the stream channel, we select the some ones
				    */
				    while let Some(message) = get_stream_messages.next().await{ 
	
                            let resp_val = message.unwrap();
                            let stringified_new_commit_topic = String::from_resp(resp_val).unwrap();

                            // self.execute(stringified_new_commit_topic.as_bytes()).await;
				    
				        }
			   
		    		}
	    
	    		});
		    

		    Ok(())
		    
	    
	    }
	}

}

pub async fn race_condition_avoidance(){

    /* ---------------------------------------------------------------------- */
    /* ---------------------- RACE CONDITION AVOIDANCE ---------------------- */
    /*  more info in: https://github.com/wildonion/zoomate/blob/main/src/dp.rs
    
        race conditions means that two threads want to mutate the data 
        at the same time, we have to use mutex so tell the other threads
        wait there is a threads that is trying to mutate this type and 
        will update you once the lock gets freed and in order to avoid blockcing 
        issues in the current thread we have to lock inside a separate thread 
        and mutate the type in there like tokio::spawn() then send it through 
        the jobq channel to the other threads for reading and future mutations
    */
    
    pub type ArcedMutexed<'lifetime> = std::sync::Arc<tokio::sync::Mutex<String>>;
    
    #[derive(Clone)]
    pub struct Data<D: Send + Sync + 'static>{
        /* we're using tokio mutex to avoid blocing issues inside the current thread since it locks asycnly */
        pub actual: D
    }
    let mut data_instance = Data::<ArcedMutexed>{
        actual: std::sync::Arc::new(
            tokio::sync::Mutex::new(
                String::from("a mutexed data")
            )
        ),
    };
    
    println!("data instance actual value before getting mutated >>> [{}]", data_instance.actual.lock().await.to_owned());
    
    /* reading from the channel is a mutable process thus receiver must be mutable */
    let (data_sender, mut data_receiver) = 
        tokio::sync::mpsc::channel::<Data<ArcedMutexed>>(1024);
    /*
        since tokio spawn takes a closure which captures the env vars 
        we have to use the cloned form of those types and pass them into
        the closure scopes so we can use them in later scopes 
    */
    let sender = data_sender.clone();
    tokio::spawn(async move{
        
        let new_string = String::from("an updated mutexed");
        /* 
            we're cloning data_instance and data_instance_cloned.actual to create a 
            longer lifetime value to use the cloned form to mutate, since by sending 
            data_instance_cloned to the channel its lifetime will be dropped and its 
            ownership will be moved because we're borroing the actual field by locking 
            on it so we can't move the data_instance_cloned into the mpsc channel using 
            the sender, in other words we can't move out of the type if it's behind a 
            shared reference we have to either pass a reference or clone the type and 
            work on the cloned form like the followings which we're cloning the actual 
            field to lock on its mutex and send the data_instance_cloned into 
            the downside of the channel
        */
        let data_instance_cloned = data_instance.clone();
        let data_instance_cloned_actual = data_instance_cloned.actual.clone();
        let mut data_string = data_instance_cloned_actual.lock().await; /* lock the mutex to mutate it */
        
        /* 
            mutating the locked mutex is done by dereferencing the guard 
            we're mutating data string inside the actual field in data_instance_cloned
            this will mutate the actual field inside data_instance_cloned 
        */
        *data_string = new_string; /* the actual field of the data_instance_cloned will be mutated too */

        if let Err(why) = sender.send(data_instance_cloned).await{
            println!("can't send because {:?}", why.to_string());
        }

    });

    /* receiving asyncly inside other threads to avoid blocking issues on heavy computations */
    tokio::spawn(async move{
        /* receving data asyncly while they're comming to the end of mpsc jobq channle */
        while let Some(data) = data_receiver.recv().await{
            
            let new_data_string = data.actual.lock().await.to_owned();
            println!("data instance actual value after getting mutated >>> [{}]", new_data_string);
    
        }
    });

}

pub mod network{

    pub mod rpc{
        
    }

    pub mod tcp{

    }

    pub mod udp{

    }

    pub mod http{

    }

    pub mod ws{
        
    }

    pub mod libp2p{
        
    }

}