

</> ram and cpu concepts: redispubsub and libp2p, ?async realtime pubsub streaming and push notif with aws, docker, rust and go:
    === app state, lib crates, static lazy arc mutex, ownbor wiki + ltg, zero copy, null pointer opt, serding function
    === ltg pointers, &'v, box, rc, refcell, arc, mutex, pin to break cycle and borrow types in graph, thread_local, unique storage key
    === phantomdata, boxpin, arena glob alloc, -> impl Interface, param: impl Interface, Box<dyn Trait> object safe trait for dynamic dispatching
    === time, arc, tcp, spawn, select, mpsc, mutex, whileletsome, customerror, redis timecache, thread::spawn, tokio::spawn, rayon::spawn, mailbox mpsc
    === Rust in
        game, cpu and ram ----> ltg, pointer, graph algos (ownership sharing by cloning using rc, arc, mutex, refcell, box, pin)
        web               ----> actix ws (Stream), http (Multipart), actor worker (redis pubsub, borker pubsub, gossipsub p2p rpc) 
        networking        ----> tokio(time,channel,spawn,tcp,select,arc,mutex,rwlock), libp2p, redis, tonic grpc actors
        wasm, lle and ffi ----> tauri, yew, bindgen, bpf
        cryptography      ----> wallexerr (high entropy seed -> hash seed -> generate rng from seed -> generate keypair)
    chat and realtime push notif strategy using actor workers and redis over http/ws/tcp:
        0) every actor worker object is an event handler
        1) app state to start s3, actors and configs
        2) http requset or ws stream: Payload command to reveal or update some data through http request then update data in db
        3) publisher actor or the emitter worker usually the redis actor publishes new data to redis channel inside the tokio spawn time loop until a subscriber receives it 
        4) subscriber actor worker is constantly subscribing in the background with ctx.run_interval, redis async and while let some to receive the data
        5) cache received data from redis some where or update the state of the actor itself or the system actor using actix broker internal pubsub to store the received data
        7) in http or ws endpoint retrieve data from the cached storage or reading the actor state itself or the system actor by sending a message to it to fetch data 
        7) from the http endpoint send notifications data received from the cacher related to the passed in entity id
        8) from the ws stream: Payload channel send the notifications data received from the cacher through the open connection back to the caller
        9) create then send sse to client through /events/ route with a broadcaster chanenl struct
    chat and realtime push notif strategy using tcp based remote actor workers:
        0) every actor worker object is an event handler
        1) send data between two actor workers in a cluster directly by calling each others methods
        2) method calling can be done through the grpc or actix telepathy p2p gossipsub protocols
        3) streaming over bytes can be done inside ctx.run_interval with while let some 
    chat and realtime push notif strategy using redis or libp2p and actor workers:
        0) every actor worker object is an event handler
        1) send data between two actor workers using redis pubsub workers or p2p gossipsub
        2) every actor is both a publisher/emitter and subscriber at the same time
        3) communication between actors can be done through redis pubsub channels or libp2p gossipsub protocols