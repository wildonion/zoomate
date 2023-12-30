
---------------------------------------------------------------------
JSON, Multipart, Protobuf, Payload based http, tcp, ws and rpc server
---------------------------------------------------------------------

WebRTC, ffmpeg and GStreamer for audio and video streaming, codec and compressing
setup oauth2 with yew ssr feature for wasm based manit dashboard
compile to wasm using: wasm-bindgen, wasmer, wasi, wasmtime

https://drive.google.com/file/d/1-8M8BNMabNPzPZM43ekWqX_D456KaUvT/view => the programmer guides to theory
https://drive.google.com/file/d/14l2B6cdAECz_tIRtQtkf2iYxnc5pDv9S/view?usp=drive_link => distributed patterns in backend
https://github.com/MoonKraken/youtube/tree/main/KonaaAuth
https://github.com/wildonion/gem/tree/master/core/panel/events => websocket implementations
https://crates.io/crates/capnp-rpc
https://github.com/actix/examples/tree/master
https://github.com/actix/examples/tree/master/protobuf
https://github.com/actix/examples/blob/master/websockets/chat-tcp/src/codec.rs => run session actor in a separate tokio::spawn thread using tcp server and custom codec
https://github.com/wildonion/cs-concepts
https://github.com/wildonion/cs-concepts#-blogs-and-books
https://github.com/wildonion/cs-concepts/blob/main/backend-roadmap.pdf
https://connectivity.libp2p.io/
https://blog.cloudflare.com/rust-nginx-module/
https://github.com/wildonion/uniXerr/blob/master/infra/valhalla/coiniXerr/src/tlps/p2p.pubsub.rs
https://github.com/libp2p/rust-libp2p/tree/master/examples
https://github.com/foniod/build-imageshttps://www.qualcomm.com/content/dam/qcomm-martech/dm-assets/documents/RaptorQ_Technical_Overview.pdf
https://docs.peer5.com/guides/production-ready-hls-vod/
https://blog.tempus-ex.com/hello-video-codec/
https://stackoverflow.com/a/56475851
https://www.quora.com/How-do-you-write-a-video-codec
https://coaxion.net/blog/2017/07/writing-gstreamer-applications-in-rust/
https://github.com/security-union/rust-zoom
https://999eagle.moe/posts/rust-video-player-part-1/
https://ffplayout.github.io/
https://bparli.medium.com/adventures-in-rust-and-load-balancers-73a0bc61a192
https://github.com/jsdw/weave
https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs
https://github.com/hyperium/hyper/blob/master/examples/gateway.rs
https://dzone.com/articles/rust-based-load-balancing-proxy-server-with-async
https://truelayer.com/blog/grpc-load-balancing-in-rust
https://medium.com/load-balancer-series/writing-a-http-load-balancer-in-python-using-tdd-theoretical-concepts-fb6dab3e879b
https://kemptechnologies.com/load-balancer/load-balancing-algorithms-techniques
https://github.com/bparli/convey
https://github.com/NicolasLM/nucleon


rust cli zoomate features and ownership, borrowing rules:
    - .wasm, .so, libp2p, redis, tokio and actix tools for event driven, graph and distributed based streaming nodes using wallexerr and hadead
    - multithreaded and async node, agent and balancer engines with blockchain graph based distributed algorithms and scheduling tlps using:
        > actor based coding for async message sending and realtime stream message handlers and listeners using
            ------------------------------------------------------------------------------------------------------------
            --------------- actor based push notif pubsub macros for streaming over topics in a same app ---------------
            ------------------------------------------------------------------------------------------------------------
            actors and dsl based local and tcp based pubsub macroes for realtime streaming,
            push notifs and monitoring like grafana with distributed algos with:
            tokio tcp listener, mpsc, select, spawn, time
            actix web apis, shared state data between routers threads using app data
            actix web stream: Payload, payload: Multipart, data: web::Josn<Data>
            actix ws actor, broker 
            tonic grpc protobuf
            Lazy<Arc<Mutex<MapDbOrConfiFile>>> + Send + Sync + 'static
            extract multipart into map then convert it into serde json then map json value into structure 
            or just serde json value if we don't know the incoming type of data from any server 
            local and tcp based pubsub actor streaming using mpsc and redis, libp2p and sqlx:
            1 - start both redis and subscriber actor globally 
            2 - PUBLISHER MACRO === publish/fire/emit topic using redis actor where an event must be triggered 
            3 - SUBSCIBER MACRO === in subscriber actor interval in tokio::spawn start subscribing using while let some syntax
            4 - client/server can be an actor and can stream over incoming packets and topics from server response/client response
            pubsub realtime monitoring, streaming and push notification over a receiver/subscriber/listener with:
            • actor based pubsub workers in server/client (like tcp,tonic,http) for realtime streaming over receiver/subscriber and monitoring like grafana
            • start actors globally in a place when the server is being built
            • share the started actor between threads as an app data state in this case the data must be Arc<Mutex<Actor>> + Send + Sync + 'static
            • initialize a global in memory map based db using static Lazy<Arc<Mutex<Actor>>> send sync 'static
            • local pubsub pattern (using actix actor worker and the broker crate with mpsc channel)
                publisher actor  ➙ publish/fire/emit/trigger event data using actix broker 
                subscriber actor ➙ stream/subscribe over/to incoming message data from publisher in an interval in tokio::spawn while let some and mpsc
            • redis pubsub pattern
                publisher actor  ➙ publish/fire/emit/trigger event data using redis actor in an interval then break once a subscriber receives it
                subscriber actor ➙ stream/subscribe over/to incoming stringified data from redis in an interval in tokio::spawn while let some and mpsc
            • tokio tcp streaming pattern
                publisher actor  ➙ publish/fire/emit/trigger event data using tokio tcp client actor
                subscriber actor ➙ stream/subscribe over/to incoming utf8 data from client in an interval in tokio::spawn while let some and mpsc
            • actix ws http streaming pattern
                publisher actor  ➙ publish/fire/emit/trigger event data using ws client actor
                subscriber actor ➙ stream/subscribe over/to incoming stream: Payload, payload: Multiaprt data from client in an interval in tokio::spawn while let some and mpsc
            • http api must be triggered by frontend every 5 seconds in which we send message to subscriber actor worker to 
              get all user notifications from redis and send it as the json response back to the caller
            -----------------------------------------------------------------------------------------------------------
            --------------- actor based push notif pubsub macro for streaming over topics in a two apps ---------------
            -----------------------------------------------------------------------------------------------------------
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
            > pubsub streaming inside actor done with:
            > tokio::tcp,udp,mpsc,select,spawn,time,mutex,rwlock,asynciotraits
            > actix::brokerpubsub,http,actor,ws,Multipart,Payload,Protobuf extractor,mailbox using while let Ok((stream, addr)) = listener.accept().await{}
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
   • zoomate protocol must be started with zoomate:// 
   • (DQL (ql, mdp) for data decryption like rsa, aes256, wallexerr ecc curves)
   • tokio channels + worker green threadpool + event loopg, hyper, actix actor concepts, (g)rpccapnp, zmq, libp2p stacks, ws, tcp and udp
   • distribute data by finding other nodes using kademlia algo 
   • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
   • simple-hyper-server-tls, noise-protocol and tokio-rustls to implement ssl protocols and make a secure channel for the underlying raw socket streams
   • graph algos in ai and distributed models like libp2p and ipfs
   • sign hash of aes256 bits of data using ed25519 private key then verify ed25519 sig using data hash, pubkey and sig 
   • ssl and ssh certs using ring rsa and wallexerr ed25519 ecc curve with aes256 hash of data for ssh, tcp, rpc with tokio-rustls to sign and encrypt the packets with pubkey to pass them through socket
   • gateway and proxy using actix
   • (g)rpccapnp to communicate between each balancer
   • decompress encoded packet using borsh and serde 
   • cpu task scheduling, 
   • vod streaming
   • weighted round robin dns, 
   • vector clock, 
   • event loop
   • A* to find a path to a node in the whole network
   • iptables and ssh tunneling
   • zmq pub/sub with borsh serialization 
   • simd divide and conquer based vectorization using rayon multithreading
   • with rayon each vector can be analyzed in a separate thread then with mpsc channels we can gather them all together in a single vector
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