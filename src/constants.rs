


use crate::*;


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


/*  ----------------------------------------------------------------------
    implementing a dynamic type handler for structs and enums using traits
    ----------------------------------------------------------------------
*/
trait TypeTrait{
    type Value;

    fn get_data(&self) -> Self::Value;
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
}