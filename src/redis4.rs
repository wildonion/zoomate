/*

    following is a simple implementation of an in-memory data storage like redis
    in which a data packets will be received by the server in an async and concurrent 
    manner then we'll send it to an mpsc jobq channel to be stored inside a sharded 
    hash map which will be updated asyncly and concurrently during the lifetime of the 
    app server we can extend this to have actors instead of functions.

    the shared data must be an arced mutex or Send + Sync in order to share
    it between tokio green threadpool, in the following we're designing a 
    shared state sharding to decrease the time lock of the muted, we can use 
    a shard from the pool to update the mutex by locking on it inside a thread 
    (either blocking using std::sync::Mutex or none blocking using tokio::sync::Mutex) 
    and if other thread wants to use it, it can use other shard instead of waiting 
    for the locked shard to gets freed, we can use try_lock() method to check 
    that the shard is currently being locked or not also we have to update the 
    whole shards inside the pool at the end of each mutex free process which is 
    something that will be taken care of by semaphores, also we've used tokio 
    mutex to lock on the mutex asyncly instead of using std mutex which 
    is a blocking manner.

    # 🇸4 (Sharded Shared State Storage)

    a very simple implementation of an in-memory data storage like redis which is 
    based on sharded shared sate storage design pattern using standard `HashMap` 
    as the shared data


    🛠️ Tools 

    * tokio select to receive data from jobq channels asyncly 
    * tokio spawn to handle incoming packets asyncly and concurrently
    * tokio mutex to acquire the lock on the shared data between tokio green threads asyncly
    * tokio jobq channels to move data between tokio green threads asyncly

    🚧 WIPs

    * implement proper sharding and replication algorithms like assigning each data of a 
    shard to a slot owned by a node by sending them through the threads using jobq channels 

*/


use redis::{RedisError, AsyncCommands};

use crate::*;


/* 
    this is an in memory data storage with a unique i32 
    storage key which will be create randomly also it must 
    be safe to be shared between threads or must be
    Arc<tokio::sync::Mutex<Db>> in which an arced and mutexed
    data is also bounded to Send and Sync traits and has a 
    valid lifetime across threads like 'static
*/
pub const LAZY_STATIC_SHARED_DATA: Lazy<Db> = Lazy::new(||{
    HashMap::new()
});

type Db = HashMap<i32, String>; 

pub const SHARDS: u32 = 10;

pub struct Response;
pub struct Request;

#[derive(Debug, Serialize, Deserialize)] // required to be implemented to unwrap() the sender result after awaiting
pub struct Data{id: String}
  



pub async fn start_server<F, A>(mut apifunc: F, redis_pubsub_msg_sender: tokio::sync::mpsc::Sender<String>, redis_client: redis::Client) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    where F: FnMut(Request, Response) -> A + Send + Sync + 'static + Clone,
    A: futures_util::future::Future<Output=Result<Response, ()>> + Send + Sync + 'static
    {
    
    /* ----------------------------------------------------------------------------------------------- */
    /* ----------------------------------------------------------------------------------------------- */
    /* ----- TOKIO TCP SOCKET SERVER SHARED STATE EXAMPLE TO HANDLE INCOMING CONNECTIONS ASYNCLY ----- */
    /* ----------------------------------------------------------------------------------------------- */
    /* ----------------------------------------------------------------------------------------------- */
    
    let mut rand_generator = Arc::new(tokio::sync::Mutex::new(ChaCha12Rng::from_entropy()));
    let (mutex_data_sender, mut mutex_data_receiver) = tokio::sync::mpsc::channel::<Db>(SHARDS as usize);
    let (mut map_shards_sender, mut map_shards_receiver) = tokio::sync::broadcast::channel::<Vec<Arc<tokio::sync::Mutex<Db>>>>(SHARDS as usize);
    

    let send_sync_map = Arc::new(tokio::sync::Mutex::new(LAZY_STATIC_SHARED_DATA.clone())); //// no need to put in Mutex since we don't want to mutate it
    let mut mutex_data_sender = mutex_data_sender.clone();      
  
    /*
        
        initializing the map shards so we can store all
        the mutexed db instances in there and udpate it 
        during the lock acquisition inside the app.

    */
    let mut map_shards = vec![send_sync_map.clone(); SHARDS as usize]; /* filling the vector with 10 shards */
    let mut current_data_length = map_shards[0].lock().await.len();
    
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Arc<tokio::sync::Mutex<Data>>>(1024);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2001").await.unwrap();
    
    info!("🚀 redis4 server is started at at 0.0.0.0:2001");
    
    /* 
        once we run this method and get to here, this tokio::spawn contains 
        the whole streaming logic which will be executed in the background 
        hence allows the code to be finished executing and have no constant 
        listening in the terminal, the solution to this is to make the app 
        alive and don't let it to be finished so we can monitor the process
        inside this tokio::spawn while the app is running, this can be done
        by adding a loop{} after calling the start_streaming() method which
        allows the app to be ran constantly and prevent finishing execution
    */
    tokio::spawn(async move{
            
        // handle streaming async tasks like accepting packets from a socket connections in a none blocking
        // manner or asyncly and concurrently can be done using tokio::spawn(async move{}) also moving 
        // shared state data between tokio::spawn() green threadpool is done using jobq channels 
        // and between clusters using redis and routers' threads by putting the data in arc, mutex and rwlock 
        // which forces us to have a Send + Sync + 'static data in the meanwhile handle incoming async 
        // events into the server can be done using tokio::select!{} eventloop. 

        while let Ok((mut stream, addr)) = listener.accept().await{ 
            
            // discord message queue cache send message from bot asyncly even after the bot has started.
            // ...
            // since in all http servers every router api is an async task that contains a 
            // request handler which must handle the incoming tcp request like parseing in 
            // tokio green threadpool thus the data that must be shared between these apis 
            // must be Arc<tokio::sync::Mutex<Data>> + Send + Sync + 'static 
            // ...
            // parse request data and share it between 
            // different threads using tokio jobq channels
            // ...

            /* cloning senable and syncable data here to not to lose their ownership in each iteration */
            let mut apifunc = apifunc.clone();
            let sender = sender.clone();
            let redis_pubsub_msg_sender = redis_pubsub_msg_sender.clone();
            let redis_client = redis_client.clone();

            
            /*  
                to avoid blocking issues we must put every heavy async task 
                inside tokio spawn to be handled asyncly in the background.

                also we're handling apis of each connection 
                inside tokio green threadpool asyncly to avoid
                halting issues
            */
            tokio::spawn(async move{
            
                let mut buffer = vec![0 as u8; 1024];
                while match stream.read(&mut buffer).await{
                    Ok(size) if size == 0 => false,
                    Ok(size) => { // &buffer[..size] gives us all the packets that socket server is received till now
                        
                        /* deserialize the packet comming from the socket connection */
                        let mut data = serde_json::from_slice::<Data>(&buffer[..size]).unwrap();
                        
                        /* update data_ structure */
                        data.id = uuid::Uuid::new_v4().to_string();
                        
                        /* sending data to the redis pubsub channel */
                        info!("📢 publishing new data to redis pubsub [data] channel");
                        let string_data = serde_json::to_string_pretty(&data).unwrap();
                        let mut conn = redis_client.get_async_connection().await.unwrap();   
                        let _: Result<_, RedisError> = conn.publish::<String, String, String>("data".to_string(), string_data.clone()).await;

                        /* sending data to the redis pubsub mpsc channel */
                        redis_pubsub_msg_sender.send(string_data.clone()).await;

                        /* making the data type, sendable and syncable by putting it in Arc and Mutex */
                        let mut data_ = Arc::new(tokio::sync::Mutex::new(data));
    
                        /* calling the api of this connection, return response and close the connection */
                        // - a lazy response object and mutating it inside routes using mutex 
                        // - returning a new response object everytime once the scoket gets disconnected (0 cost type)
                        // let resp = api(Request{}, Response{}).await.unwrap();
                        // let encoded_resp_object = serde_json::to_vec(&resp).unwrap();
                        // stream.write_all(&encoded_resp_object).await.unwrap();
                        // stream.shutdown().await;

                        /* calling the api of this connection */
                        apifunc(Request{}, Response{}).await.unwrap();
                        
                        /* sending data_ to the down side of the channel */
                        sender.send(data_.clone()).await.unwrap();
                        
                        true
                    },
                    Err(e) => {
                        
                        info!("➔ terminating connection {}", stream.peer_addr().unwrap());
                        
                        /* http server closes the connection after handling each task */
                        if let Err(e) = stream.shutdown().await{
                            error!("➔ error in closing tcp connection");
                        }
                        
                        false
                    }
                } {} // this is for while

            });

        }
    });

    /* waiting inside the eventloop to receive the shared data asyncly once the data sent to channel */
    tokio::select!{
        data = receiver.recv() => {
            if let Some(mut d) = data{
                
                /* pass the data to the s4 to store in ram */
                
                /* 
                    if we pass a mutable pointer to the type to the method calls then by mutating the 
                    pointer inside the method the value of that type outside the method will be mutated too.

                    currently map_shards and current_data_length will be mutated concurrently 
                    thus we've passed a mutable borrow to them to the sharded_shared_state method
                */
                sharded_shared_state_storage(
                    d,
                    &mut map_shards,
                    rand_generator, 
                    mutex_data_sender,
                    mutex_data_receiver,
                    map_shards_sender,
                    map_shards_receiver,
                    &mut current_data_length,
                ).await;

                /* 
                    after this function execution we have a mutated and updated map_shards
                    &mut current_data_length in this method since we've passed their mutable
                    borrow to the sharded_shared_state method in which if the borrow or the 
                    pointer gets mutated we have a mutated value of these types in this method.
                */
            }
        }
    }

    /* ----------------------------------------------------------------------------------------------- */
    /* ----------------------------------------------------------------------------------------------- */
    /* ----------------------------------------------------------------------------------------------- */


    Ok(())

}


pub async fn sharded_shared_state_storage(
    data: Arc<tokio::sync::Mutex<Data>>,
    map_shards: &mut Vec<Arc<tokio::sync::Mutex<HashMap<i32, String>>>>, /* will be mutated */
    rand_generator: Arc<tokio::sync::Mutex<ChaCha12Rng>>, 
    mutex_data_sender: mpsc::Sender<HashMap<i32, String>>,
    mut mutex_data_receiver: mpsc::Receiver<HashMap<i32, String>>,
    map_shards_sender: broadcast::Sender<Vec<Arc<tokio::sync::Mutex<HashMap<i32, String>>>>>,
    mut map_shards_receiver: broadcast::Receiver<Vec<Arc<tokio::sync::Mutex<HashMap<i32, String>>>>>,
    current_data_length: &mut usize, /* will be mutated */
){


    /*

        waiting to receive the new shards from the channel 
        to update the current shard inside the whole app
        asyncly, since we're inside an eventloop this can 
        be done at any time inside the app thus we're sure
        that we'll always use an udpated version of the shards 

    */
    tokio::select!{ //// instead of using while let ... syntax on the receiver
        sent_shards = map_shards_receiver.recv() => {
            if let Ok(shards) = sent_shards{
                *map_shards = shards; /* updating with the incoming shards */
                let deref_map_shards = (*map_shards).clone();
                *current_data_length = deref_map_shards[0].lock().await.len();
            }
        }
    }


    /*

        after finding a free mutex we'll update it then send it to the 
        downside of the mpsc job queue channel in order to update the vector 
        of shards by selecting the largest mutex data, this must be done asyncly 
        since we don't know the time of the lock acquisition, it can happen any 
        time during the app and due to having a shard of mutex data we have to 
        update the whole shards with the latest data in a none blocking manner
        since there might be other mutex-es that are in a lock process.    
    
    */
    let map_shards = map_shards.clone();
    tokio::spawn(async move{
        let generator = rand_generator.clone(); 
        for idx in 0..map_shards.clone().len(){
            match map_shards[idx].clone().try_lock(){
                Ok(mut gaurd) => {

                    // generate random number
                    let mut rng = generator.lock().await;
                    let random = rng.to_owned().gen::<i32>();

                    // udpate the gaurd by inserting a new random unique storage key
                    let data_val = &data.lock().await.id; /* borrowing the data to prevent from moving in each iteration of the loop */
                    let value = format!("value is {}", data_val);
                    gaurd.insert((idx as i32) * random, value);

                    // send the mutex to downside of the channel
                    mutex_data_sender.send(gaurd.to_owned()).await.unwrap();

                },
                Err(e) => {
                    // use other mutex instead
                    continue;
                } 
            }
        }
    });
    
    /* 

        in here we're waiting to receive the mutex data
        from the channel asyncly in order to update shards
        based on the largest mutex data to remove forks.  

    */
    let current_data_length = current_data_length.clone();
    tokio::spawn(async move{
        tokio::select!{ //// instead of using while let ... syntax on the receiver
            mutex_data = mutex_data_receiver.recv() => {
                if let Some(largest_data) = mutex_data{
                    
                    // check that this is the largest data
                    if current_data_length < largest_data.len(){
                        
                        // update the whole shards with the largest_data
                        let new_shards = vec![Arc::new(tokio::sync::Mutex::new(largest_data)); SHARDS as usize];

                        // broadcast the new shards to the channel so all receivers can use the updated version
                        map_shards_sender.send(new_shards).unwrap();
                        
                    } else{

                        /* MEANS THAT NO MUTEX HAS BEEN MUTATED YET */
                        // ...
                    }

                } else{
                    
                    /* SOMETHING WENT WRONG IN SENDING TO CHANNEL */
                    // ...
                }    
            }
        }
    });

}