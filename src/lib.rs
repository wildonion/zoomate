

use std::collections::HashMap;
use actix::{Actor, Handler, Message, StreamHandler};
use actix_web::HttpResponse;
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
use hadead::*;
use once_cell::sync::Lazy;
use base64::{engine::general_purpose, Engine as _};
use wallexerr::misc::*;
use sha3::{Digest, Keccak256, Keccak256Core};
use ring::rand as ring_rand;
mod constants;
use constants::*;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet as Set;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Expr, Ident, Local, Pat, Stmt, Token, FnArg};
mod misc;
use misc::*; // load all macros



// global rate limiter
pub static HADEAD: Lazy<Config> = Lazy::new(||{

    let redis_password = "geDteDd0Ltg2135FJYQ6rjNYHYkGQa70".to_string();
    let redis_username = "".to_string();
    let redis_host = "localhost".to_string();
    let redis_port = "6379".to_string();
    let chill_zone_duration_in_seconds = 5;

    let hadead_instance = hadead::Config::new(
        &redis_password,
        &redis_username,
        &redis_host,
        &redis_port,
        chill_zone_duration_in_seconds, /* default is 5 miliseconds */
    );

    hadead_instance

});


/* 
    the ok arm of return type is an HttpResponse object which can be 
    parsed in any server or client application and can be sent through
    the tcp socket to either ckient or seerver
*/
#[post("/api")]
pub async fn api(req: HttpRequest, stream: Payload) -> Result<actix_web::HttpResponse, actix_web::Error>{

    /* we have to fill a buffer on server with incoming bytes by streaming over `stream` object */
    let mut bytes = vec![];
    while let Some(item) = stream.next().await {
        bytes.extend_from_slice(&item?);
    }

    let hadead = HADEAD.clone();
    println!("hadead contract info: {:?}", hadead.contract.as_ref().unwrap());

    let check_rate_limited = hadead.check(hadead.id.as_ref().unwrap()).await;
    
    let Ok(flag) = check_rate_limited else{
        
        let why = check_rate_limited.unwrap_err();
        return Ok(
            HttpResponse::NotAcceptable().json(why.to_string())
        );
    };

    if flag{

        // rate limited

        return Ok(
            HttpResponse::NotAcceptable().json("rate limited")
        );

    } else{

        tokio::spawn(async move{

            // other api logics
            // ...
        
        });

        return Ok(
            HttpResponse::Ok().json("some json data")
        );

    }

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

pub fn sign_with_ed25519(data: &str, mut wallet: Wallet) -> String{
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

pub async fn set_response<'lifetime, G, T: Send + Sync + 'static + FnMut() -> G>
    /* since T is a FnMut closure, the cls param must be defined mutablly */
    (mut cls: T){

    {
        let data = constants::GLOBAL_MUTEXED.clone();
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
		pub pid: String, // keccak256 bits hash of the whole data
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

pub async fn start_tcp_listener(){
        
    #[derive(Default, Serialize, Deserialize, Debug, Clone)]
    pub struct TcpServerData{
        pub data: String,
    }
    let tcp_server_data = TcpServerData::default();
    let (tcp_msg_sender, mut tcp_msg_receiver) = 
        tokio::sync::mpsc::channel::<String>(1024);
        
        /* ----------------------------------------- */
        /* starting a tcp listener in the background */
        /* ----------------------------------------- */

        let bind_address = format!("0.0.0.0:2323");
        let mut api_listener = tokio::net::TcpListener::bind(bind_address.as_str()).await;
        let (job_sender, mut job_receiver) = tokio::sync::mpsc::channel::<String>(1024);

        let api_listener = api_listener.unwrap();
        println!("➔ 🚀 tcp listener is started at [{}] to accept streaming of utf8 bytes", bind_address);

        tokio::spawn(async move{

            println!("➔ start receiving asyncly and concurrently in tokio green threadpool");

            /*  -------------------------- STREAMING NOTES --------------------------
            |   streaming can be done using actix|tokio|tonic with tlps like wsactor|http|tcp|grpc in a separate 
            |   threadpool like in tokio::spawn(), actors in a same server can use actix and tokio stuffs to send/recv 
            |   responses actors in two different mses can use tcp, (g)capnprpc or redis to send/recv responses also 
            |   there must be a message and stream handlers implemented for actors so they can communicate with each 
            |   other and different parts of the app to send/receive static lazy mutex streams of utf8 bytes data based 
            |   on serde_json, web::Payload, Multipart and capnp+protobuf codecs throught rpc or mpsc channel based on 
            |   tokio::spawn,mpsc,mailbox,mutex,select,time, we can also have a pubsub pattern for them using 
            |   libp2pgossipsub,rpc,redisstreamqueue,actixbroker pubsub

                also see extractor::multipart() which handles incoming multipart form data asyncly 
                by streaming over each field to gather the field's bytes then map it into a 
                data type or serde json value
                
                streaming over a realtiming source like a socket to fill the buffer with incoming u8 
                future byte objs chunks and then map into a struct can be done with tokio(mpsc,select,spawn,
                mutex,rwlock,tcp) actix-ws-http,redis&libp2ppubsub and can be a webhook/stream/event 
                handler which will accept streaming of events' data utf8 bytes can be like:  

                let (data_sender, mut data_receiver) 
                    = tokio::sync::mpsc::channel::<std::sync::Arc<tokio::sync::Mutex<Data>>>(1024);
                let buffer = vec![];
                let mut bytes = web::BytesMut::new();
                let streamer_body: web::Payload;
                tokio::task::spawn(async move{ 
                    while let Some(chunk) = streamer_body.next().await{
                        let byte = chunk.as_slice();
                        buffer.extend_from_slice(byte);
                        bytes.extend_from_slice(byte);
                        let decoded_data = serde_json::from_slice::<Data>(&buffer).unwrap();
                        data_sender.clone().send(
                            std::sync::Arc::new(
                                tokio::sync::Mutex::new(
                                    Some(decoded_data)
                                )
                            )
                        ).await;
                    }
                });
                while let Some(received_data) = data_receiver.recv().await{
                    let mut data = received_data.lock().await;
                    *data = Default::default();
                }

                the nature of rust codes are not asynced and multithreaded by default we must use
                a runtime for that like tokio and run async tasks inside tokio::spawn() threadpool
                which takes care of running an async context in a free thread behind the scene and 
                won't let other codes in other scopes get halted and waited for this job to be
                finished and, they get exectued on their own without blocking the scopes  
                thus if we have a condition like
                if condition {
                    return something to the caller;
                }

                the rest of the code after if won't get executed with this nature we can 
                only have one if, provided that it terminate the method body with an statement,
                and respond the caller with a value; once the body get terminated the rest of
                the code won't be executed cause we don't have async context by default, 
                other than that we have to provide the else part since rust needs to know 
                that if not this type then what type?!
            */
            // streaming over incoming bytes to fill the buffer and then map the buffer to structure 
            while let Ok((mut api_streamer, addr)) = api_listener.accept().await{
                println!("🍐 new peer connection: [{}]", addr);

                // cloning those types that we want to move them into async move{} scopes
                // of tokio::spawn cause tokio::spawn will capture these into its closure scope
                let tcp_server_data = tcp_server_data.clone();
                let job_sender = job_sender.clone();

                tokio::spawn(async move {

                    /* this buffer will be filled up with incoming bytes from the socket */
                    let mut buffer = vec![]; // or vec![0u8; 1024] // filling all the 1024 bytes with 0

                    while match api_streamer.read(&mut buffer).await { /* streaming over socket to fill the buffer */
                        Ok(rcvd_bytes) if rcvd_bytes == 0 => return,
                        Ok(rcvd_bytes) => {
                
                            let string_event_data = std::str::from_utf8(&buffer[..rcvd_bytes]).unwrap();
                            println!("📺 received event data from peer: {}", string_event_data);

                            /*  
                                sending the decoded bytes into the mpsc channel so we could receive it  
                                in other scopes or threads
                            */
                            if let Err(why) = job_sender.send(string_event_data.to_string()).await{
                                eprintln!("❌ failed to send to the mpsc channel; {}", why);
                            }
                    
                            let send_tcp_server_data = tcp_server_data.data.clone();
                            if let Err(why) = api_streamer.write_all(&send_tcp_server_data.as_bytes()).await{
                                eprintln!("❌ failed to write to api_streamer; {}", why);
                                return;
                            } else{
                                println!("🗃️ sent {}, wrote {} bytes to api_streamer", tcp_server_data.data.clone(), send_tcp_server_data.len());
                                return;
                            }
                        
                        },
                        Err(e) => {
                            eprintln!("❌ failed to read from api_streamer; {:?}", e);
                            return;
                        }
                        
                    }{}
            
                });
            }{}
        
        });

        tokio::spawn(async move{

            /* 
                write the incoming data from channel to file constanly 
                as they're coming from the mpsc channel 
            */
            let f = tokio::fs::File::open("readmeasync.txt").await;
            if let Err(why) = f.as_ref(){
                println!("can't create file cause: {}", why.to_string());
            }
            let mut funwrapped = f.unwrap();

            while let Some(job) = job_receiver.recv().await{

                if let Err(why) = funwrapped.write(job.as_bytes()).await{
                    println!("can't write to file cause: {}", why.to_string());
                }
            
            }
            
        });
	
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



/*  > -------------------------------------------
    |           proc macro functions 
    | ------------------------------------------
    |
    |   RUST CODES ---> TOKEN STREAM ---> AST
    |
    | 0 - compiler generates TokenStreams of Rust codes that this proc macro is placed of top of
    | 1 - parse (a new parser perhaps!) TokenStreams (Rust codes) to generate AST using syn
    | 2 - write new Rust codes using the patterns inside the generated AST like mutating idents or variables
    | 3 - convert generated or mutated either pure Rust codes in step 2 into a new AST using quote
    | 4 - return the new AST as a new TokenStream to the compiler to update the method or struct field at compile time
    |


    https://veykril.github.io/tlborm/introduction.html
    https://blog.logrocket.com/procedural-macros-in-rust/
    https://danielkeep.github.io/tlborm/book/README.html


    since macro processing in Rust happens after the construction of the AST, as such, 
    the syntax used to invoke a macro must be a proper part of the language's syntax 
    tree thus by adding a new code in this crate the compiler needs to compile the whole 
    things again, which forces us to reload the workspace everytime, means by that any 
    logging codes don't work in here at runtime and we must check them in console once 
    the code gets compiled.

    a TokenStream is simply built from the Rust codes which can be used to built the
    AST like: RUST CODES ---> TOKEN STREAM ---> AST also the following are matters:
    sync generates : the AST from the passed in TokenStream (sequence of token trees)
    quote generates: Rust codes that can be used to generate TokenStream and a new AST

    proc macro can be on top of methods, union, enum and struct and can be used to add 
    method to them before they get compiled since compiler will extend the struct AST 
    by doing this once we get the token stream of the struct Rust code. it can be used 
    to parse it into Rust pattern (ident, ty, tt and ...) that will be used to add a new 
    or edit a logic on them finally we must use the extended token stream of the Rust codes 
    that we've added to convert them into a new token stream to return from the macro to 
    tell the compiler that extend the old AST with this new one

    kinds: 
        decl_macro (inside misc.rs)
        proc_macro
        proc_macro_derive
        proc_macro_attribute
    
    benefits:
        add a method to struct or check a condition against its fields
        convert trait into module to extend the trait methods
        extend the interface of a struct by changing the behaviour of its fields and methods
        create a DSL like jsx, css or a new keyword or a new lang
        build a new AST from the input TokenStream by parsing incoming tokens and return the generated TokenStream from a new Rust codes
        write parser using decl_macro
        changing and analysing the AST logics of methods at compile time before getting into their body
        bind rust code to other langs and extending code in rust using macros
    

*/





struct Args{
    vars: Set<Ident>
}

/*
    we need to create our own parser to parse the 
    args token stream into a new AST
*/
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

/*
    with the following proc macro we can do inspect and operate on the 
    api methods before generating the output or executing any extra
    logics before getting into the api body like actix #[get()] which
    checks the request path in the first place before sliding into 
    the request body, also to get the Rust token codes from TokenStream 
    we must use syn::parse and to get the TokenStream from Rust codes 
    we msut use quote
*/
#[proc_macro_attribute]
pub fn passport(args: TokenStream, input: TokenStream) -> TokenStream {

    /*  
        build the new AST from the `input` TokenStream to extend the one that we 
        already have by using syn to parse the args & input tokens into a syntax 
        tree, note that the type of TokenStream that we want to parse it with syn 
        to generate AST, must be specified, like parsing a function TokenStream 
        into ItemFn AST, then we need to generate a new TokenStream from generated 
        Rust types parsed from the input TokenStream, using quote to do so, generate 
        a new TokenStream from the passed in Rust codes (either pure or using #variable) 
        to it that can be used to build a new AST, this will replace whatever `input` 
        is annotated with this attribute proc macro, finally we'll return the token 
        stream either generated by the quote or the passed in input.

        when we are defining a procedural macro, we're not actually interacting with 
        the runtime data, instead, we're generating code that will be inserted into 
        the function thus we can't access the token inside the request object in this 
        proc macro since procedural macros work at compile time, they don't have access 
        to runtime data, in our case, the token in the HTTP request header is available 
        at runtime, so it's impossible to directly inspect the header's content inside
        a procedural macro.
    */
    let mut api_ast = syn::parse::<syn::ItemFn>(input.clone()).unwrap(); /* parsing the input token stream or the method into the ItemFn AST */
    let roles_set = parse_macro_input!(args as Args).vars; /* casting the args TokenStream into the Args parser */
    let mut granted_roles = vec![];
    for role in roles_set{
        granted_roles.push(role.to_string()); /* converting the Ident into String */
    }

    /*  
        every variable can be shown as ident in Rust thus if we wanna have a new variable we must 
        create new ident instance, like the following for the request object, also every token 
        in a TokenStream has an associated Span holding some additional info, a span, is a region 
        of source code, along with macro expansion information, it points into a region of the 
        original source code(important for displaying diagnostics at the correct places) as well 
        as holding the kind of hygiene for this location. The hygiene is relevant mainly for 
        identifiers, as it allows or forbids the identifier from referencing things or being 
        referenced by things defined outside of the invocation.
    */
    let mut req_ident = syn::Ident::new("req", proc_macro2::Span::call_site());
    for input in api_ast.clone().sig.inputs{
        if let FnArg::Typed(pat_type) = input{
            if let Pat::Ident(pat_ident) = *pat_type.pat{
                if pat_ident.ident.to_string() == "req".to_string(){
                    req_ident = pat_ident.ident;
                    break;
                }
            }
        }
    }

    /* 
        generating a token stream from granted_roles variable, 
        quote generates new AST or token stream from Rust codes
        that can be returned to the proc macro caller.
    */
    let new_stmt = syn::parse2(
        quote!{ /* building new token stream from the Rust token codes */
            
            /* 
                granted_roles can be accessible inside the api body at runtime, 
                vec![#(#granted_roles),*] means that we're pushing all the roles
                inside a vec![] and since there are multiple roles we used * to 
                push them all into the vec![] which means repetition pattern
            */
            let granted_roles = vec![#(#granted_roles),*];

        }
    ).unwrap();

    /* inject the granted_roles into the api body at compile time */
    api_ast.block.stmts.insert(0, new_stmt); // extending the AST of the api method at compile time
    
    /* 
        return the newly generated AST by the quote of the input api Rust code  
        which contains the updated and compiled codes of the function body
    */
    TokenStream::from(quote!(#api_ast)) /* building new token stream from the updated api_ast Rust token codes */


}


#[proc_macro]
pub fn fn_like_proc_macro(input: TokenStream) -> TokenStream {

    // ex:
    // #[macro_name]
    // fn im_a_method(){}
    
    input

}

#[proc_macro_derive(Passport)]
pub fn derive_proc_macro(input: TokenStream) -> TokenStream {

    // ex:
    // #[derive(Passport)]
    // struct SexyStruct{}
    
    // this will be implemented in here for the struct inside input token stream
    // so later on we can call the method on the struct once we've implemented the
    // method for the struct in here.
    // SexyStruct::passport() 

    input

}