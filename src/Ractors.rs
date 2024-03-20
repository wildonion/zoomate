


/* 

actors have:
mailbox channel, parallelism[thread::spawn, tokio::spawn, rayon::spawn], 
redis pubsub, borker pubsub, gossipsub p2p grpc tonic

in essence if each actor would be a controller then with this design pattern we can communicate 
with each controller remotely it's like calling smart contract methods from other contracts, in 
a high insight every api node actor can communicate with each other inside a cluster through rpc
p2p gossipsub and redis pubsub to aware each others of joining new nodes

         __________________________________ CLUSTER ____________________________________
        |                                                                               |
  admin component node actor                                                    user component node actor 
                |                                                                           |
                 ----------                                                       ---------- 
                           |                                                     |
                            --------remotely------- || --------locally ----------
                                        |                         |
                                        |                         |
                                        |                         |
                          rpc,p2pgossipsub,redispubsub        broker,mpsc
*/

// remote actors
pub mod balancer; // load name of the folder cause it has mod.rs inside