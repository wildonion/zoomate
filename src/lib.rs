

/*

---------------------------------------------------------------------
JSON, Multipart, Protobuf, Payload based http, tcp, ws and rpc server
---------------------------------------------------------------------

https://github.com/wildonion/gem/tree/master/core/panel/events => websocket implementations
https://crates.io/crates/capnp-rpc
https://github.com/actix/examples/tree/master
https://github.com/actix/examples/tree/master/protobuf
https://github.com/actix/examples/blob/master/websockets/chat-tcp/src/codec.rs => run session actor in a separate tokio::spawn thread using tcp server and custom codec
https://github.com/wildonion/cs-concepts
https://github.com/wildonion/cs-concepts#-blogs-and-books
https://connectivity.libp2p.io/
https://blog.cloudflare.com/rust-nginx-module/
https://github.com/wildonion/uniXerr/blob/master/infra/valhalla/coiniXerr/src/tlps/p2p.pubsub.rs
https://github.com/libp2p/rust-libp2p/tree/master/examples
https://github.com/foniod/build-images
https://www.qualcomm.com/content/dam/qcomm-martech/dm-assets/documents/RaptorQ_Technical_Overview.pdf



rust cli zoomate features and ownership, borrowing rules:
    - multithreaded and async node, agent and balancer engines with blockchain distributed algorithms and scheduling tlps using:
        > actor based coding for async message sending and realtime stream message handlers and listeners using
            > --------------------------------------------------------------------------------------------
            | with actors we can communicate between different parts of the app by sending async 
            | messages to each other through jobq channels, they also must have a handler for each 
            | type of incoming messages like redis streams and pubsub patterns with ws actors and 
            | tokio concepts (jobq channels, spawn, select, time interval) by streaming over io 
            | future object of bytes to register a push notif also protobuf is an IDL based and 
            | serding data structure that can be used between services to send and receive packets
            | based on the same structure which has been defined in proto file, each structure is an
            | actor object which allows services to call each structure's method directly through the rpc 
            | protocol without having any extra api and packet handlers , they can be used to handle 
            | message streaming in realtime and bi directional manner
            > ------------------------------------------------------------------
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
             --------------------------------------------------------------------------------------------
            > pubsub and streaming be like:
            > tokio::tcp,udp,mpsc,select,spawn,time,mutex,rwlock,asynciotraits,mailbox using while let Ok((stream, addr)) = listener.accept().await{}
            > actix::brokerpubsub,http,actor,ws,Multipart,Payload,Protobuf extractor
            > libp2p::dht,kademlia,gossipsub,noise protocol,quic,tokio::tcp,p2pwebsocketwebrtc
            > redis::pubsub,streams,queue
            > tonio::grpc::protobuf,capnp
            > then:
                1 - static lazy mutexed, rusty ltg pointers, box pin trait and stackless, ret ref and slice from method, &mut type, codec, async io traits then coerce heap data to slice form, pass slice form in method param
                2 - gathering incoming bytes to fill the buffer by streaming over the source asyncly in a threadpool
                3 - decode the gathered bytes into desire structure or form of data (protobuf, bson, serdecodec, multipart, payload)
                    let buffer: Vec<u8>;
                    let json_data = serde_json::to_value(&buffer[..]).unwrap();                    ----- convert buffer slice into json data (useful when want to send and parse response as json value instead of mapping into structure)
                    let json_string = serde_json::to_string_pretty(&buffer[..]).unwrap();          ----- convert buffer slice into json stringify
                    let data_from_slice = serde_json::from_slice::<DataBucket>(&buffer).unwrap();  ----- convert buffer slice into structure instance
                    let data_from_str = serde_json::from_str::<DataBucket>(&json_string).unwrap(); ----- convert json stringified into structure instance
                    let data_str_from_slice = std::str::from_utf8(&buffer[..]).unwrap();           ----- convert buffer slice into &str when we can't map it into an structure (useful when we're receiving unstructured data from a source)
                    let name = "wildonion";
                    let hex_name = hex::encode(name.as_bytes());
                    let decode_hex_name = hex::decode(hex_name).unwrap();
                    let real_name = std::str::from_utf8(&decode_hex_name).unwrap();
                4 - share the Arc<Mutex<DataBucket>> between threads using mpsc jobq channel
                5 - receiving data inside other threads from the mpsc receiver using while let Ok(data) = receiver.recv().next().await{} syntax
        > node/agent/bot (
                > note that agent is an async and multithreaded based clinet&&server
                > note that kademlia will be used to find nodes on the whole network
            )
            |
            |
             ---actix-wss/tokio mutex,select,jobq,spawn,tcp,udp)/(g)rpccapnp/actix-https
                libp2p quic,gossipsub,kademlia,noise/redis pubsub strams
                noise,tokio-rustl,wallexerr,web3
                                |
                                |
                                 --- node/agent/bot
        --=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--
    - event driven architecture:
        tcp and websocket webhook/stream/event handler for realtiming push notif to get the inomcing 
        bytes like streaming tlps over image chunks (call next on it and async read/write traits 
        must be used) from a source to store in a buffer then map the buffer into a struct using
        tokio(time,spawn,select,mutex,tcp,jobq) to avoid deadlocks and race conditions and using 
        actix_web_actor::ws and actix actors then we can update some logic based on the caught events 
        and notify other parts of the app, threads and scopes by publishing the event as a notification
        using redis pubsub so other parts and microservices can subscribe to that, webhook means once 
        an event gets triggered an api call will be invoked to notify (it's like a notification to the server) 
        server about the event happend as a result of handling another process in some where like a 
        payment result in which server subscribes to incoming event type and can publish it to 
        redispubsub so other app, threads and scopes can also subscribe to it 
        --=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--
    - stream/wh/event handler and loop to stream over incoming future io utf8 byte objects as payload, asyncly 
        > note that we must implement and StreamHandler trait for every
            ws actor struct so it can handle the incoming utf8 bytes from
            a client in realtime since actor execute streaming of async 
            tasks in their threadpool and we can send results between different 
            parts of the app and other actors by pre defined message passing logic
            based on mpsc jobq channel so the streaming syntax is as follows:
            tokio::spawn(async move{

                let mut bytes = web::BytesMut::new();
                let mut buffer = vec![];

                ////---------------------------------------------------------
                ////---- handling websocket payload in an actix route
                ////---------------------------------------------------------
                while let Some(item) = payload_stream.next().await {
                
                    bytes.extend_from_slice(&item?);
                
                }

                ////-----------------------------------------------
                ////---- handling an stream from a socket 
                ////-----------------------------------------------
                
                while let Some(chunk) = streamer.chunk().await? {
                
                    buffer.extend_from_slice(chunk);
                    // decod buffer into struct as they're coming 
                    // ...
        
                }  

            });


1) a realtime and pluging based node monitoring and packet sniffing tools which
can heal itself using a DL based algo on top of transformers and VAE techniques
using tokio/redis/actix/zmq/(g)rpccapnp/libp2p to manage the load of each instance 
in realtime, in our proxy, zmq subscribers are server app node instances 
that must be balanced by subscribing on the incoming topic from the balancer 
publishers, like spread requests between node server instances using different 
balancing algorithms and pubsub pattern to manage the total load of the VPS 
also we can build zmq using tokio socket actors and build libp2p and rpc 
system using zmq pub/sub sockets,

2) using rusty ltgs pointers (https://github.com/wildonion/rusty/blob/main/src/retbyref.rs#L17), 
hadead and wallexerr in ssh login, api rate limiting and webhook registery, updating app and server 
verification apis async stream/event bytes handler using tokio stuffs/actix ws actor/redispubsub 
for parallel tasks and storing unique encrypted data on with global data[arcmutexrwlock concept 
based on .so and .wasm vms also unique assets and nodes detection by feature extraction algos 
like VAE in such a way that we must generate a vector of 8000 numbers of each node or 
assets using VAE latent space then compare the node with incoming nodes to check that 
if they're unique or not also a packet loss correction engine like raptorq forward 
error correction system to reconstruct the packets using VAE and transformers in video 
and audio streaming (raptor.rs)

3) codec like serde, borsh, protobuf and capnp also send notif (publish backonline topic) 
to other pods if another one gets back online or finding online pods 
using following flow:
    - actix ws actor event and stream handler/loop using tokio spawn, 
        select, mpsc, mutex and tcp with redis and libp2p pubsub streams
    - event and stream handler to handle the incoming async task like ws 
        messages packets using actix StreamHandler and tokio tcp 
    - message handler to handle the message type which is going to 
        be sent between other actors and other parts of the app
    - ws actor stream and event handlers are like:
        streaming over incoming bytes through the tokio tcp socket 
        to send them as the async task to tokio green threadpool using
        tokio spawn to handle them as an event using tokio select event 
        loop handler

	>>>> look start_tcp_listener() method <<<<
	streaming over incoming encoded io future object of utf8 bytes 
 	using actix actor ws/(g)rpccapnp/tcp/http and tokio(tcp,spawn,jobq mpsc,select,time,mutex,rwlock)
	to decode them into structs to mutate them concurrently by moving
	them between tokio threads using jobq channels and mutex 
			    or 
	event of async task handler, streamer, loop 
	inside std::thread::scope and tokio::spawn based 
	tokio tcp stream or mmq streaming over future 
	bytes using tokio and ws actor and redis pubsub 
	and streams by streaming over incoming bytes 
	inside the tokio gread threadpool and pass them 
	to other threads using tokio::sync::mpsc, actor, 
	select, spawn, mutex, pubsub, tcp stream, hex, serding 
	to_string vs from utf8

4)‌ bpf based proxy, firewall, vpns, packet sniffer and load balancer like pingora, docker networking, nginx, ngrok, HAproxy, v2ray and wireshark for all layers
   • tokio channels + worker green threadpool + event loopg, hyper, actix actor concepts, (g)rpccapnp, zmq, libp2p stacks, ws, tcp and udp
   • distribute data by finding other nodes using kademlia algo 
   • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
   • simple-hyper-server-tls, noise-protocol and tokio-rustls to implement ssl protocols and make a secure channel for the underlying raw socket streams
   • gateway and proxy using actix
   • (g)rpccapnp to communicate between each balancer
   • decompress encoded packet using borsh and serde 
   • cpu task scheduling, 
   • vod streaming
   • weighted round robin dns, 
   • vector clock, 
   • event loop
   • iptables and ssh tunneling
   • zmq pub/sub with borsh serialization 
   • simd divide and conquer based vectorization using rayon multithreading (each vector can be analyzed in a separate thread)
   • language binding
   • reverse proxy for NAT traversal implemented in Rust based macros
   • implement DNS Server in Rust (DNS hijacking and spoofing using mitm tools)
   • a dns server like docker to map the dns to the container ip in host
   • google Search Crawler implemented in Rust (scalable and secure)
   • caching server implemented in Rust like redis
   • scalable and Secure Firewall implemented in Rust
   • ngrok process: [https://docs.rs/ngrok/latest/ngrok/] || [https://ngrok.com/docs/using-ngrok-with/rust/]
 	➙ first it'll open a port on local machine 
 	➙ then it will create a session on that port with a random dns on its servers 
 	➙ finally it forwards all the traffic to that session to the local port it created
	➙ ngrok and ssh vps will starts a server on a random part then forward all the packets 
 	  coming from outside to the localhost it's like: 
	  outside <---packet---> ngrok or ssh vps server act like proxy <---packet---> localhost
   • cloudflare warp vpn
	    • boringtun protocol which is based on wireguard protocol
	    • uses noise protocol with ed25519 encryption
	    • 1111 dns based protocol 
	    • udp and quic for packet sending   
	    • argo routing to send packets to cloudflare gateways
	    • ed25519 digital signature pubkey with chacha20 in noise protocol for making vpn
   • VPS configuration according to the source usage of each node 
    ➙ like dpi to detect anomal packets to coiniXerr server and automatic load balancer and vps config using transformers and drl
    ➙ OS and a security management app(malware detection) using RL
    ➙ our VPS must detect the amount of CPU and RAM that every servers needs to get, without running the app
    ➙ our VPS must detect the number of instances of every servers needs to be run and the load balancing algorithm
*/


use std::collections::HashMap;
use actix::{Actor, Handler, Message, StreamHandler};
use actix_web::HttpResponse;
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
use wallexerr::{DataBucket, Contract, Wallet};
use sha3::{Digest, Keccak256, Keccak256Core};
use ring::rand as ring_rand;
mod constants;
use constants::*;



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
pub async fn api() -> Result<actix_web::HttpResponse, actix_web::Error>{

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

    pub async fn proof_of_chain(chain_addresses: Vec<String>, chain_addresses_from_other_servers: Vec<String>){

        /*      ------ good for whilelist ------
            having two different mutable pointer to instances are not allowed in a single scope
            based on this we're ok to call calculate_root_hash() method which takes a mutable pointer 
            of the struct instance method two times on the same instance 
        */
        let mut merkle_tree_wl = constants::MerkleNode::new();
        let old_merkle_hash = merkle_tree_wl.calculate_root_hash(chain_addresses_from_other_servers);  
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

                also see multipartreq crate in gem which handles incoming multipart form data asyncly 
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