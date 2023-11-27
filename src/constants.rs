


use rand::{Rng, random};
use crate::*;

pub const CHARSET: &[u8] = b"0123456789";

/*      
           --------------------------------------------------
                             Mutex Vs RwLock
           --------------------------------------------------


    Mutex (Mutual Exclusion Lock):

    A Mutex allows only one thread to access some data at any given time.
    Every time a thread wants to access the data, it must first lock the Mutex.
    If you have a situation where you have more writes than reads, or if the 
    reads and writes are roughly equal, a Mutex is usually the better choice.
    tokio::sync::Mutex is an asynchronous Mutex that is designed to work 
    with async code within tokio.
    
    RWLock (Read-Write Lock):

    A RWLock allows any number of threads to read the data if there isn't a thread writing to it.
    If a thread wants to write to the data, it must wait for all the readers to finish before it 
    can obtain the lock. If you have a situation where you have many more reads than writes, a RWLock 
    can be more efficient because it allows multiple readers to access the data simultaneously.
    tokio::sync::RwLock is an asynchronous Read-Write Lock that works within the async ecosystem of tokio.
    
    When to choose tokio::sync::Mutex:

    You have frequent writes.
    The critical section (the part of the code that needs exclusive access to the data) is quick.
    You want simplicity. Using a Mutex is straightforward and avoids the complexity of dealing with 
    multiple lock types.
    
    When to choose tokio::sync::RwLock:

    You have many more reads than writes.
    You want to allow concurrent reads for efficiency.
    The critical section for reads is fast, but it’s still beneficial to have multiple readers at the same time.
    In many scenarios, a Mutex might be sufficient and can be the simpler choice, especially if write operations 
    are as common as reads or if the critical section is very short, thus not justifying the overhead of managing 
    a RWLock.

    However, if your specific case involves a lot of concurrent reads with infrequent writes and the read operations 
    are substantial enough to create a bottleneck, a RWLock might be a better choice.


*/

/*      
           --------------------------------------------------
                 a thread safe global response objects
           --------------------------------------------------

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
                with this in mind there is no need to define a global mutexed response object
                and reload it everytime in each api to avoid runtime memory overloading, cause rust
                will handle this automatically by using the concept of borrowing and lifetime

        No Garbage Collector: While the presence or absence of a garbage collector (GC) isn't the 
                main reason Rust is cautious with global state, it's worth noting. Many languages 
                with GCs allow for more liberal use of global state because the GC can clean up. 
                In Rust, manual memory management means you need to be more careful.

    heap data need to be behind pointer or their slice form, for traits they must be behind Box<dyn or 
    &dyn, for String and Vec need to be &str and &[], good to know that if we want to return pointer to 
    heap data from method they must be in their slice form with a valid lifetime like pointer to closure 
    traits as method param and return type, if the heap data is one of the field of a structure it's ok 
    to return a pointer with a valid lifetime to that cause the pointer will be valid as long as the &self 
    is valid and once we move self into a new scope like tokio::spawn() we can't do this although rust won't
    allow move self to new scope in the first place, in general we shouldn't return pointer to type from 
    method since rust handled each type lifetime automatically unless it's a mutable pointer cause mutating 
    a mutable pointer will mutate the actual type

    global state of type requires to have a complex valid lifetime like 'static and be mutable which this can't 
    be happend since rust doesn't have gc logic to track the lifetime of the type based on the references to that 
    type although it has Rc and Weak which can be used to count the references of a type but instead it uses the 
    concept of borrowing and ownership which is about destroying the type and drop its lifetime from the ram once 
    the type goes out of the scope like by moving heap data types into a function scope in essence by mutating an 
    static lifetime type we may face deadlock and race conditions issues in other threads, instead we can define 
    an static mutex since static types are immutable by default and because static values must be constant we must 
    put the mutex inside Lazy, like the following:
    since we can't return none const from a static type thus we have to 
    put it inside the lazy as a closure which returns the actual type 
    because Arc and RwLock are none const types although we can implement 
    this logic using thread_local!{}, see https://github.com/wildonion/gvm/edit/main/src/lib.rs

    note that the data we want to share it between threads must be Send + Sync + 'static
    eg: Lazy<std::sync::Arc<tokio::sync::RwLock<ZoomateResponse>>> + Send + Sync + 'static 
    as a mutable global data will be shared between apis to mutate it safely to avoid deadlocks 
    and race conditions and the sharing process can be done using mpsc jobq channel sender
    so having this: 
    	 // can't put the actual data in const since Arc and RwLock are none const types that can mutate data
    	pub static MULTI_THREAD_THINGS: std::sync::Arc<tokio::sync::RwLock<Vec<u8>>> = 
     		std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new()));
    is wrong and we should use the following syntaxes instead:
*/

pub static ZOOMATE_RESPONE_STORAGE: Lazy<std::sync::Arc<tokio::sync::Mutex<ZoomateResponse>>> = 
Lazy::new(||{
    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            ZoomateResponse{
                data: String::from("")
            }
        )
    )
});

pub static GLOBAL_MUTEXED: Lazy<std::sync::Arc<tokio::sync::Mutex<HashMap<u32, String>>>> = 
Lazy::new(||{ 
    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            HashMap::new()
        )
    ) 
});

/* 
    an static global mutex must be in RwLock in order to be mutable safely in threadpool 
    since static types can't be mutated since rust doesn't have gc and by mutating an static
    type we might have race conditions in other scopes.
*/
pub static USER_RATELIMIT: Lazy<HashMap<u64, u64>> = Lazy::new(||{
    // futures are trait that must be behind a valid pointer 
    // and pinned to ram to solve them later in other scopes
    let fut = 
        async move{
            String::from("")
        };

    let fut_: std::pin::Pin<Box<dyn std::future::Future<Output=String>>>
        = Box::pin(
            fut 
        );
    HashMap::new()
});

// following is incorrect since std::sync::Arc<tokio::sync::RwLock<Lazy<String>>>
// is not constant and the whole type must be wrapped into the Lazy<>
/* 
pub static MUTABLE_USER_RATELIMIT: std::sync::Arc<tokio::sync::RwLock<Lazy<String>>> = 
    std::sync::Arc::new(
        tokio::sync::RwLock::new(
            /* 
                since USER_RATELIMIT is not constant we can't have it here because
                static types must have constant values
            */
            Lazy::new(||{String::from("")})
        )
    );
*/


#[derive(Debug, Clone)]
pub enum RuntimeCode{
    Err(u8),
    Ok(u8),

}

pub struct MerkleNode{}
impl MerkleNode{

    pub fn new() -> Self{
        MerkleNode {  }
    }

    pub fn calculate_root_hash(&mut self, chain: Vec<String>){

    } 
}

/* converting an slice array of u8 bytes into an array with 32 byte length */
pub fn convert_into_u8_32(data: &[u8]) -> Option<[u8; 32]>{
    data.try_into().ok()
}

pub fn gen_random_chars(size: u32) -> String{
    let mut rng = rand::thread_rng();
    (0..size).map(|_|{
        /* converint the generated random ascii to char */
        char::from_u32(rng.gen_range(33..126)).unwrap() // generating a char from the random output of type u32 using from_u32() method
    }).collect()
}

pub fn gen_random_number(from: u32, to: u32) -> u32{
    let mut rng = rand::thread_rng(); // we can't share this between threads and across .awaits
    rng.gen_range(from..to)
} 

pub fn gen_random_idx(idx: usize) -> usize{
    if idx < CHARSET.len(){
        idx
    } else{
        gen_random_idx(random::<u8>() as usize)
    }
}

pub fn string_to_static_str(s: String) -> &'static str { 
    /* 
        we cannot obtain &'static str from a String because Strings may not live 
        for the entire life of our program, and that's what &'static lifetime means. 
        we can only get a slice parameterized by String own lifetime from it, we can 
        obtain a static str but it involves leaking the memory of the String. this is 
        not something we should do lightly, by leaking the memory of the String, this 
        guarantees that the memory will never be freed (thus the leak), therefore, any 
        references to the inner object can be interpreted as having the 'static lifetime.
        
        also here it's ok to return the reference from function since our reference lifetime 
        is static and is valid for the entire life of the app

        leaking the memory of the heap data String which allows us to have an 
        unfreed allocation that can be used to define static str using it since
        static means we have static lifetime during the whole lifetime of the app
        and reaching this using String is not possible because heap data types 
        will be dropped from the heap once their lifetime destroyed in a scope
        like by moving them into another scope hence they can't be live longer 
        than static lifetime

        Note: this will leak memory! the memory for the String will not be freed 
        for the remainder of the program. Use this sparingly
    */
    Box::leak(s.into_boxed_str()) 

}

/* 
*/
pub fn vector_to_static_slice(s: Vec<u32>) -> &'static [u32] { 
    /* 
        we cannot obtain &'static str from a Vec because Vecs may not live 
        for the entire life of our program, and that's what &'static lifetime means. 
        we can only get a slice parameterized by Vec own lifetime from it, we can 
        obtain a static str but it involves leaking the memory of the Vec. this is 
        not something we should do lightly, by leaking the memory of the Vec, this 
        guarantees that the memory will never be freed (thus the leak), therefore, any 
        references to the inner object can be interpreted as having the 'static lifetime.
        
        also here it's ok to return the reference from function since our reference lifetime 
        is static and is valid for the entire life of the app

        leaking the memory of the heap data Vec which allows us to have an 
        unfreed allocation that can be used to define static str using it since
        static means we have static lifetime during the whole lifetime of the app
        and reaching this using Vec is not possible because heap data types 
        will be dropped from the heap once their lifetime destroyed in a scope
        like by moving them into another scope hence they can't be live longer 
        than static lifetime

        Note: this will leak memory! the memory for the Vec will not be freed 
        for the remainder of the program. Use this sparingly
    */
    Box::leak(s.into_boxed_slice()) 

}


/*  ----------------------------------------------------------------------
    implementing a dynamic type handler for structs and enums using traits
    ----------------------------------------------------------------------
*/
trait TypeTrait{
    type Value;

    fn get_data(&self) -> Self::Value;
    fn get_ctx_data(&self, ctx: Self::Value) -> Self;
    fn fill_buffer(&mut self) -> Vec<u8>;
}

impl TypeTrait for MerkleNode{
    
    type Value = std::sync::Arc<tokio::sync::Mutex<HashMap<u32, String>>>;

    fn get_data(&self) -> Self::Value {
        
        let mutexed_data = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                HashMap::new()
            )
        );
        mutexed_data
    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn fill_buffer(&mut self) -> Vec<u8> {
        todo!()
    }
}

struct Streamer;
struct Context<T>{data: T}
impl TypeTrait for Streamer{
    
    type Value = Context<Self>; /* Context data is of type Streamer */

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        ctx.data
    }

    fn get_data(&self) -> Self::Value {
        todo!()
    }

    fn fill_buffer(&mut self) -> Vec<u8> {
        todo!()
    }

}

impl TypeTrait for RuntimeCode{
    
    type Value = std::sync::Arc<tokio::sync::Mutex<String>>;
    
    fn get_data(&self) -> Self::Value {
        
        let mutexed_data = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                String::from("")
            )
        );
        mutexed_data

    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn fill_buffer(&mut self) -> Vec<u8> {
        todo!()
    }
}

pub trait NodeReceptor{
    type InnerReceptor;
    fn get_inner_receptor(&self) -> Self::InnerReceptor;
}

pub trait Activation<C>: Send + Sync + 'static + Clone + Default{
    type Acivator;
}

impl<C> Activation<C> for &'static [u8]{
    type Acivator = &'static [u8];
}

#[derive(Default)]
pub struct Synapse<A>{id: A}

#[derive(Default)]
pub struct Neuron<A=u8>{
    pub data: Option<Synapse<A>>,
    pub multipart: Option<actix_multipart::Multipart>,
    pub payload: Option<actix_web::web::Payload>,
}

/* 
    this must be implemented for Neuron<Synapse<A>>
    to be able to call get_inner_receptor() method
*/
impl<A: Default> NodeReceptor for Neuron<Synapse<A>>
where Self: Clone + Send + Sync + 'static + Activation<String>, 
<Self as Activation<String>>::Acivator: Default{

    type InnerReceptor = Synapse<A>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        let id: A = Default::default();
        Synapse{
            id,
        }
    }
}

/* 
    this must be implemented for Neuron<String>
    to be able to call get_inner_receptor() method
*/
impl NodeReceptor for Neuron<String>{

    type InnerReceptor = Synapse<String>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        Synapse{
            id: String::from(""),
        }
    }
}

/* 
    this must be implemented for Neuron<A>
    to be able to call get_inner_receptor() method
*/
impl NodeReceptor for Neuron<u8>{

    type InnerReceptor = Synapse<u8>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        Synapse{
            id: 0,
        }
    }
}

pub fn fire<'valid, N, T: 'valid + NodeReceptor>(cmd: N, cmd_receptor: impl NodeReceptor) 
-> <N as NodeReceptor>::InnerReceptor // or T::InnerReceptor
where N: Send + Sync + 'static + Clone + NodeReceptor + ?Sized, 
T: NodeReceptor, T::InnerReceptor: Send + Clone,
<N as NodeReceptor>::InnerReceptor: Send + Sync + 'static{

    let pinned_boxed_future: std::pin::Pin<Box<dyn std::future::Future<Output=String>>> = 
        Box::pin(async move{
            String::from("")
        });

    /*  
        note that if we want to call get_inner_receptor() method
        on an instance of Neuron, the NodeReceptor trait must be
        implemented for every generic type in Neuron struct separately
        like:
            impl NodeReceptor for Neuron<String>{}
            impl NodeReceptor for Neuron<u8>{}
            impl NodeReceptor for Neuron<Synapse<A>>{}
    */
    let neuron = cmd;
    let neuron_ = Neuron::<String>::default();
    
    cmd_receptor.get_inner_receptor();
    neuron.get_inner_receptor()
    // neuron_.get_inner_receptor()
    
}