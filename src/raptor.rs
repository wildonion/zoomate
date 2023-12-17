


/* 

    - help from https://github.com/wildonion/gvm/
    - pointers (&mut, shared ref, rc, arc, mutex, refcell, rwolock, as ref, can't move type, can't ret pointer, can't have more than one mutable pointers)
    - traits (box &dyn, closures, impl Trait, extend the behavior of types)
    - generics (bounding to traits and lifetimes)
    - enum, ram concepts, stack, heap, cap, length, pointer 
    - global static lazy arced mutexed/rwlocked map as a thread safe and an in memory database using multithreading and actor streaming (both in client and server) concepts with tcp based tlps:
        sqlx and tokio rustls wallexerr, redis hadead, tonic grpc, Payload, Multiaprt, Protobuf
        tokio::spawn,mutex,rwlock,mpsc,select,tcp
        actix::http,web,broker,actor
    - rusty ltg(&mut,box,pin,trait,macro,generic) and shared state data to make them shareable between routes and threads using Arc<Mutex<Db>> + Send + Sync + 'static with mpsc
    - arc will be used to share the pointer of the type between threads safely cause it's a atomically-reference-counted shared pointer
    - trigger, publish, fire, emit event means that we'll send a packet on an specific condition to a channel
        so subscribers can subscribe to and stream over that packet (mpsc receiver, file, mulipart, payload, tcp based packets) 
        using actor, tokio::spawn, while let some to fill the buffer then decode and map to struct or serde json 
        value and from other parts send message to the actor to get the decoded data
    
    
*/

use actix::prelude::*;
use crate::*;


#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Raptor{

}

impl Actor for Raptor{
    type Context = Context<Raptor>;
}


pub mod codec{

    pub async fn encoder(){
    
    }
    
    pub async fn decoder(){
        
    }
    
}


struct Pointer<'valid, T>{
    pub data: &'valid mut T
}

impl<T: Default + Clone> Pointer<'_, T>{

    /* 
        we can ret a mutable pointer in here cause we're using 
        the lifetime of the self which is valid as long as the 
        instance is valid
    */
    pub async fn register_new_pointer(&mut self) -> &mut T{
    
        self.data

    }

}

struct Struct<'valid, G>{
    pub data: &'valid G
}

impl<'g, G: Clone + Default + Send + Sync + 'static> Event for Struct<'g, G>{
    
    type Room<'valid> = std::sync::Arc<tokio::sync::Mutex<G>>;

    fn get_room<'valid>(&mut self) -> Self::Room<'valid> {

        fn get_name() -> String{ String::from("") }
        let callback = |func: fn() -> String|{
            func();
        };
        callback(get_name);
        
        let d = self.data.clone();
        std::sync::Arc::new(
            tokio::sync::Mutex::new(
                d 
            )
        )
    }

}

trait Event{
    
    type Room<'valid>: 
    'valid + ?Sized + Default + Clone + 
    Send + Sync + 'static; // we can bound the Room GAT to traits in here

    fn get_room<'g>(&mut self, cls: impl FnOnce(String) -> String) -> Self::Room<'g>;

}

// let mut struct_instance = Struct::<String>{
//   data: &String::from("")
// };
// let thetype = struct_instance.get_room();