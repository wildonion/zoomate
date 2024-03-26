


use std::io::Write;
use rand::{Rng, random};
use crate::*;

pub const CHARSET: &[u8] = b"0123456789";


#[derive(Serialize, Deserialize)]
struct TcpSharedSetup{
    pub wallet: wallexerr::misc::Wallet,
    pub secure_cell_config: wallexerr::misc::SecureCellConfig
}
pub static SECURECELLCONFIG_TCPWALLET: Lazy<(wallexerr::misc::SecureCellConfig, wallexerr::misc::Wallet)> = Lazy::new(||{
    
    let mut wallet = wallexerr::misc::Wallet::new_ed25519();
    // creating the secure cell config structure
    let mut aes256_config = &mut wallexerr::misc::SecureCellConfig::default();
    // following secret key is the sha3 keccak256 hash of random chars
    aes256_config.secret_key = {
        hex::encode(
            wallet.self_generate_keccak256_hash_from(
                &gen_random_chars(64)
            )
        )
    };
    // save the config so we can share it between clients
    let mut file = std::fs::File::create("tcp_wallet_secure_cell_config.json").unwrap();
    file.write_all(&serde_json::to_string_pretty(&TcpSharedSetup{
        wallet: wallet.clone(), 
        secure_cell_config: aes256_config.clone()
    }).unwrap().as_bytes()).unwrap();

    (aes256_config.to_owned(), wallet)
}); 

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

    
           --------------------------------------------------
                 a thread safe global response objects
           --------------------------------------------------

        single thread and none gc concepts: ltg &mut pointer (rc,refcell,arc,mutex,lazy,threadlocal),box,pin,impl Trait,

        code order execution and synchronization in multithreaded based envs like
        actor worker like having static lazy arced mutex data without having deadlocks 
        and race conditions using std::sync tokio::sync objects like 
        semaphore,arc,mutex,rwlock,mpsc also data collision, memory corruption, deadlocks 
        and race conditions avoidance in async and multithreaded contexts are: 
            - share none global app state data between tokio::spawn() threads using mpsc 
            - enum and actor id as unique storage key
            - mutate a global storage using thread local in single-threaded contexts
            - mutate a gloabl storage using static lazy arc mutexed in multi-threaded contexts
            - can't move out of a reference or deref a type if its pointer is being used by and shared with other scopes
            - can't mutate data without acquiring the lock of the mutex in other threads
            - can't have both mutable and immutable pointers at the same time 


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
    this logic using thread_local!{},

    note that the data we want to share it between threads must be Send + Sync + 'static
    eg: Lazy<std::sync::Arc<tokio::sync::RwLock<ZoomateResponse>>> + Send + Sync + 'static 
    as a mutable global data will be shared between apis to mutate it safely to avoid deadlocks 
    and race conditions and the sharing process can be done using mpsc jobq channel sender
    so having the following is wrong since the static value must be const and Arc and RwLock
    are none const types hence we must put them inside Lazy<>: 
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

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
// s3 code order execution using sync objects: 
// static lazy arced mutexed and pinned box future db type, send sync static
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
// static value must be const since we are not able to mutate its value because it's not safe
// however sharing data between threads safely requires to borrow the data to share its ownership
// using Arc and for mutation using Mutex, since these types are none const types we can use Lazy
// to make them const so we can give the static type this value.
type DbS3Type = Lazy<std::sync::Arc<tokio::sync::Mutex<
    std::pin::Pin<Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + Sync + 'static>>
    >>>;
pub static DbS3: DbS3Type = 
Lazy::new(||{
    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            Box::pin(async move{ // pinning the future object into the ram before polling it to make it as a separate object type for future solvation
                HashMap::new()
            })
        )
    )
});


// a single thread arena allocator
thread_local!{
    pub static DB: std::cell::RefCell<std::collections::HashMap<String, String>> = 
        std::cell::RefCell::new(HashMap::new());
}
// DB.with_borrow_mut(|db| {
//     db.insert("key".to_string(), "value".to_string())
// });


// following is incorrect since std::sync::Arc<tokio::sync::RwLock<Lazy<String>>>
// is not constant and the whole type must be wrapped into the Lazy<> to be a const
// value acceptable by the static cause static value must be const
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