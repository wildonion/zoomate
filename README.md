

☢️ Actor based proxy and balancer for audio and video realtiming process which can be loaded from the linux kernel using **BPF** technology

## ✨ Features

* **Cap'n Proto** as the serialization protocol

* Communication (video and audio streaming) over **ZMQ** and **RPC** based on pub/sub messaging pattern 
 
* **WebRTC**, **ffmpeg** and **GStreamer** for audio and video streaming, codec and compressing

* **P2P** streaming using **gossipsub** and **kademlia** protocols

* **Actor** and **BPF** based engine on top of **Tokio** multithreading tools (`TcpListener`, `spawn`, `select`, `Arc`, `Mutex`, `RwLock` and jobq channels)

## 📚 References
  
* https://docs.peer5.com/guides/production-ready-hls-vod/

* https://blog.tempus-ex.com/hello-video-codec/

* https://stackoverflow.com/a/56475851

* https://github.com/wildonion/cs-concepts

* https://www.quora.com/How-do-you-write-a-video-codec

* https://coaxion.net/blog/2017/07/writing-gstreamer-applications-in-rust/

* https://github.com/security-union/rust-zoom

* https://999eagle.moe/posts/rust-video-player-part-1/

* https://ffplayout.github.io/

* https://bparli.medium.com/adventures-in-rust-and-load-balancers-73a0bc61a192

* https://github.com/jsdw/weave

* https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs

* https://github.com/hyperium/hyper/blob/master/examples/gateway.rs

* https://dzone.com/articles/rust-based-load-balancing-proxy-server-with-async

* https://truelayer.com/blog/grpc-load-balancing-in-rust

* https://medium.com/load-balancer-series/writing-a-http-load-balancer-in-python-using-tdd-theoretical-concepts-fb6dab3e879b

* https://kemptechnologies.com/load-balancer/load-balancing-algorithms-techniques

* https://github.com/bparli/convey

* https://github.com/NicolasLM/nucleon