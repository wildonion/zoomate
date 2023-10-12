

/*


https://github.com/wildonion/cs-concepts
https://connectivity.libp2p.io/
https://blog.cloudflare.com/rust-nginx-module/
https://github.com/wildonion/uniXerr/blob/master/infra/valhalla/coiniXerr/src/tlps/p2p.pubsub.rs
https://github.com/foniod/build-images

event driven architecture:
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


blockchain distributed algorithms and scheduling tlps:
> note that agent is an async and multithreaded based clinet 
   node/agent/bot
          |
          |
           ---actix-wss/tokio mutex,select,jobq,spawn,tcp,udp)/rpc-capnp/actix-https
                libp2p quic,gossipsub,kademlia,noise/redis pubsub strams
			noise,tokio-rustl,wallexerr,web3
                                |
                                |
                                 --- node/agent/bot


a realtime and pluging based node monitoring and packet sniffing tools which
can heal itself using a DL based algo on top of transformers and VAE techniques
using tokio/redis/actix/zmq/rpc/libp2p to manage the load of each instance 
in realtime, in our proxy, zmq subscribers are server app node instances 
that must be balanced by subscribing on the incoming topic from the balancer 
publishers, like spread requests between node server instances using different 
balancing algorithms and pubsub pattern to manage the total load of the VPS 
also we can build zmq using tokio socket actors and build libp2p and rpc 
system using zmq pub/sub sockets  


codec like serde, borsh and capnp also send notif (publish backonline topic) 
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
 	using actix actor ws/rpc/tcp/http and tokio(tcp,spawn,jobq mpsc,select,time,mutex,rwlock)
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


sha256, sha3, Keccak256 and argon2, multipart, base64, rustls to load trusted ssl certs from /etc/ssl/certs/ca-certificates.crt 
and ssh RSA ECC curves keypair with simple-hyper-server-tls, openssl, tokio-rustls and noise-protocol we can create a secured communication 
streaming channel between our hyper, ws, tcp or udp servers and clients based on the created certificate 
and the key by implementing the tls protocols for the raw underlying 
of tcp and udp socket stream of io future objects


➙ we can setup exit codes with enum to know which error caused the program to stopped when using Box<dyn Error> which can be implemented for the type that will cause the error at runtime 
➙ public key digital signature ring ed25519 verification for updating app and server verification apis 
➙ bpf based proxy, firewall, vpns, packet sniffer and load balancer like pingora, docker networking, nginx, ngrok, HAproxy, v2ray and wireshark for all layers
   • tokio channels + worker green threadpool + event loopg, hyper, actix actor concepts, rpc capnp, zmq, libp2p stacks, ws, tcp and udp
   • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
   • simple-hyper-server-tls, noise-protocol and tokio-rustls to implement ssl protocols and make a secure channel for the underlying raw socket streams
   • gateway and proxy using hyper: https://github.com/hyperium/hyper/tree/master/examples
   • rpc capnp to communicate between each balancer
   • decompress encoded packet using borsh and serde 
   • cpu task scheduling, 
   • vod streaming
   • weighted round robin dns, 
   • vector clock, 
   • event loop
   • iptables and ssh tunneling
   • zmq pub/sub with borsh serialization 
   • simd divide and conquer based vectorization
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
*/


    
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};

mod dp;
use dp::*;

mod acter;
use acter::*;

use uuid::Uuid;





#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Request; //// it can be Option<Vec<hyper::Request<hyper::Body>>> which all the incoming http hyper requests to this node that must be handled



#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Weight{
    pub n: u16,
    pub requests: Request,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node{ //// this contains server info 
    pub dns: String,
    pub peer_id: String, 
    pub cost_per_api_call: u128, //// this is based on the load of the weights
    pub init_at: i64,
    pub weights: Option<Vec<Weight>>, //// load of requests
}


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
/* 
                a thread safe global response object  

    reasons rust don't have static global types:
        
        Memory Safety: One of Rust's main goals is to ensure memory safety without the need 
               for a garbage collector. Global state can lead to shared mutable state across 
               threads, which is a source of data races. By making global state explicit and 
               synchronized, Rust avoids these issues.

        Concurrency: Rust's concurrency model revolves around the concept of ownership. Global 
               variables can be problematic in concurrent programs, where multiple threads might 
                want to modify a global variable simultaneously.

        Predictability and Explicitness: Global mutable state can make programs unpredictable 
                and hard to reason about. Rust values explicitness over implicitness, so when you 
                see a piece of Rust code, you can easily understand its behavior without having to 
                consider hidden global states.

        Lifetimes: Rust uses lifetimes to track how long data is valid. Global state has a complex 
                lifetime that can easily lead to dangling references if not managed carefully.

        No Garbage Collector: While the presence or absence of a garbage collector (GC) isn't the 
                main reason Rust is cautious with global state, it's worth noting. Many languages 
                with GCs allow for more liberal use of global state because the GC can clean up. 
                In Rust, manual memory management means you need to be more careful.


    global state of type requires to have a complex valid lifetime like 'static 
    and be mutable which this can't be happend since rust doesn't gc and by mutating 
    an static lifetime type we may face deadlock and race conditions issues in other 
    threads, instead we can define an static mutex since static types are immutable 
    by default and because static values must be constant we must put the mutex 
    inside Lazy, like the following:
    since we can't return none const from a static type thus we have to 
    put it inside the lazy as a closure which returns the actual type 
    because Arc and RwLock are none const types although we can implement 
    this logic using thread_local!{}, see https://github.com/wildonion/gvm/edit/main/src/lib.rs

    so having this: 
    	 // can't put the actual data in const since Arc and RwLock are none const types that can mutate data
    	pub static MULTI_THREAD_THINGS: std::sync::Arc<tokio::sync::RwLock<Vec<u8>>> = 
     		std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new()));
    is wrong and we should use the following syntax instead:

*/
// note that the data we want to share it between threads must be Send + Sync + 'static
// eg: Lazy<std::sync::Arc<tokio::sync::RwLock<ResponseObject>>> + Send + Sync + 'static 
// as a mutable global data will be shared between apis to mutate it safely 
// to avoid deadlocks and race conditions
pub static RESPONE: Lazy<std::sync::Arc<tokio::sync::RwLock<ResponseObject>>> = Lazy::new(||{
    std::sync::Arc::new(tokio::sync::RwLock::new(ResponseObject::default()))
});


pub async fn set<'lifetime, G, T: Send + Sync + 'static + FnMut() -> G>
    /* since T is a FnMut closure, the cls param must be defined mutablly */
    (mut cls: T){

    /* T is a closure which returns G and can be shared between threads safely */
    let callback = cls();

    let mut res = RESPONE.write().await;
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


pub async fn agent_simulation(){

	let new_rt = tokio::runtime::Builder::new_multi_thread();
	#[derive(Clone)]
	struct BuildQueue{
		pub agent_id: String,
	}
	#[derive(Clone)]
	struct Pipeline{
		pub pid: String, // keccak256 bits hash of the whole data
	}
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
	    pub pipeline: PipeLine
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

	    async fn subscribe_to_new_commit(commit_id: &str) -> Result<(), ()>{
		    
		let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
		    tokio::spawn(async move{
			    
			    loop{

				/* tick every 15 seconds */
				interval.tick().await;
			
				    // setup async redis subscription process to subscribe to 
				    // ...
				    
				    let get_stream_messages = redis_async_pubsubconn
					.subscribe(commit_id)
					.await;
				    
				    let Ok(mut get_stream_messages) = get_stream_messages else{
					    
					return Err(());
			
				    };
				
				    /* 
					iterating through the msg future object streams as they're 
					coming to the stream channel, we select the some ones
				    */
				    while let Some(message) = get_stream_messages.next().await{ 
	
					let resp_val = message.unwrap();
					let stringified_new_commit_topic = String::from_resp(resp_val).unwrap();

					self.execute(stringified_new_commit_topic.as_bytes()).await;
				    
				    }
			   
		    		}
	    
	    		});
		    

		    Ok(())
		    
	    
	    }
	}

}


pub async fn start_tcp_listener(){

// more info in start_tcp_listener() api in gem admin access
	
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
    info!("➔ 🚀 tcp listener is started at [{}]", bind_address);

    tokio::spawn(async move{

	// streaming over incoming bytes to fill the buffer and then map the buffer to structure 
	while let Ok((mut api_streamer, addr)) = api_listener.accept().await{
	    info!("🍐 new peer connection: [{}]", addr);

	    // cloning those types that we want to move them into async move{} scopes
	    // of tokio::spawn cause tokio::spawn will capture these into its closure scope
	    let tcp_server_data = tcp_server_data.clone();
	    let job_sender = job_sender.clone();
		
	    tokio::spawn(async move {

		let mut buffer = vec![0; 1024];

		/*
                    an webhoo/stream/event handler accepts streaming of 
                    events' data utf8 bytes and can be like: 
        
                    // chunk() method returns self.streamer.body_mut().next().await;
                    tokio::spawn(async move{
                        while let Some(chunk) = streamer.chunk().await? {
                            // decod chunk into struct 
                            // ...
                        }
                    });
  	        */
		while match api_streamer.read(&mut buffer).await {
		    Ok(rcvd_bytes) if rcvd_bytes == 0 => return,
		    Ok(rcvd_bytes) => {
    
			let string_event_data = std::str::from_utf8(&buffer[..rcvd_bytes]).unwrap();
			info!("📺 received event data from peer: {}", string_event_data.clone());
			job_sender.send(string_event_data.to_string()).await;
    
			let send_tcp_server_data = tcp_server_data.data.clone();
			if let Err(why) = api_streamer.write_all(&send_tcp_server_data.as_bytes()).await{
			    error!("❌ failed to write to api_streamer; {}", why);
			    return;
			} else{
			    info!("🗃️ sent {}, wrote {} bytes to api_streamer", tcp_server_data.data.clone(), send_tcp_server_data.len());
			    return;
			}
		    
		    },
		    Err(e) => {
			error!("❌ failed to read from api_streamer; {:?}", e);
			return;
		    }
		    
		}{}
    
	    });
	}{}
	
    });


    tokio::spawn(async move{

	while let Some(job) = job_receiver.recv().await{

		// we have job in here
		// ...
	
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
        actual: std::sync::Arc::new(tokio::sync::
            Mutex::new(
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


pub mod bpf{


    /* 
    
        with BPF VM we can compile the whole node 
        into an .elf or .so which contains the 
        BPF bytecode that can be executed from 
        the linux kernel. LLVM13 is needed 
        to compile BPF bytecode for Rust version
    
        https://blog.redsift.com/labs/writing-bpf-code-in-rust/
        binding using .so and https://crates.io/crates/pyo3

    */
    
    // bpf loader
    // ... 
    
}



