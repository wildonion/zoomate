


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

pub async fn serding(){

    /* 
        future objects must be pinned to the ram before they can be solved 
        or polled the reason of doing this is first of all they're trait objects
        and traits are dynamically sized means they're size will be known at runtime
        second of all due to the fact that rust doesn’thave gc which allows us not 
        to have a tracking reference counting process for a type at runtime cause it’ll 
        move the type if the type goes of out of the scope hence in order to solve 
        and poll a future in other scopes later on, we should pin it to the ram first 
        which can be done once we await on the future but if we want to solve and poll 
        a mutable reference of a future we should stick and pin it to the ram manually, 
        first by pinning the future into the ram using Box::pin or tokio::pin!() then 
        do an await on the mutable reference of the future object, so if it is required to call 
        .await on a &mut _ reference, the caller is responsible for pinning the future
        by pinning future objects manually we make them as an object before polling 
        them like having a mutable reference to them or pass them into other parts
        to solve them in different parts
    */
    let mut future = async move{};
    tokio::pin!(future); // pinning the future object before solving/polling its mutable pointer
    let mutable_pointer = &mut future;
    mutable_pointer.await; // polling the mutable reference of the futuer object

    #[derive(Serialize, Deserialize, Debug)]
    struct DataBucket{data: String, age: i32}
    let instance = DataBucket{data: String::from("wildonion"), age: 27};
    ///// encoding
    let instance_bytes = serde_json::to_vec(&instance);
    let instance_json_string = serde_json::to_string_pretty(&instance);
    let instance_str = serde_json::to_string(&instance);
    let isntance_json_value = serde_json::to_value(&instance);
    let instance_json_bytes = serde_json::to_vec_pretty(&instance);
    let instance_hex = hex::encode(&instance_bytes.as_ref().unwrap());
    ///// decoding
    let instance_from_bytes = serde_json::from_slice::<DataBucket>(&instance_bytes.as_ref().unwrap());
    let instance_from_json_string = serde_json::from_str::<DataBucket>(&instance_json_string.unwrap());
    let instance_from_str = serde_json::from_str::<DataBucket>(&instance_str.unwrap());
    let isntance_from_json_value = serde_json::from_value::<DataBucket>(isntance_json_value.unwrap());
    let instance_from_hex = hex::decode(instance_hex.clone()).unwrap();
    let instance_from_hex_vector_using_serde = serde_json::from_slice::<DataBucket>(&instance_from_hex);
    let instance_from_hex_vector_using_stdstr = std::str::from_utf8(&instance_from_hex);
    let instance_from_vector_using_stdstr = std::str::from_utf8(&instance_bytes.as_ref().unwrap());
    
    println!(">>>>>>> instance_hex {:?}", instance_hex);
    println!(">>>>>>> instance_from_bytes {:?}", instance_from_bytes.as_ref().unwrap());
    println!(">>>>>>> instance_from_json_string {:?}", instance_from_json_string.unwrap());
    println!(">>>>>>> instance_from_str {:?}", instance_from_str.unwrap());
    println!(">>>>>>> isntance_from_json_value {:?}", isntance_from_json_value.unwrap());
    println!(">>>>>>> instance_from_hex_vector_using_serde {:?}", instance_from_hex_vector_using_serde.unwrap());
    println!(">>>>>>> instance_from_vector_using_stdstr {:?}", instance_from_vector_using_stdstr.unwrap());
    println!(">>>>>>> instance_from_hex_vector_using_stdstr {:?}", instance_from_hex_vector_using_stdstr.unwrap());

} 

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
    this logic using thread_local!{}, see https://github.com/wildonion/gvm/edit/main/src/lib.rs

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

pub static IN_MEMORY_DB: Lazy<std::sync::Arc<tokio::sync::Mutex<HashMap<u32, String>>>> = 
Lazy::new(||{ 
    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            HashMap::new()
        )
    ) 
});

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
// s3 code order execution using sync objects: 
// static lazy arced mutexed and pinned box future db type, send sync static
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
pub static Db: Lazy<std::sync::Arc<tokio::sync::Mutex<
    std::pin::Pin<Box<dyn std::future::Future<Output = HashMap<u32, String>> + Send + Sync + 'static>>
    >>> = 
Lazy::new(||{

    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            Box::pin(async move{
                HashMap::new()
            })
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
        = Box::pin( // pinning the future into the ram to make it as an on object before polling it
            fut 
        );
    HashMap::new()
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

    /* 
        we can use the lifetime of self in struct and trait methods 
        to return pointer since the self is valid as long as the object 
        itself is valid during the execution of the app
    */
    fn get_data(&self) -> Self::Value;
    fn get_ctx_data(&self, ctx: Self::Value) -> Self;
    fn fill_buffer(&mut self) -> &[u8];
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

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }
}

struct Streamer;
struct Context<T>{data: T}
impl TypeTrait for Streamer{ // polymorphism
    
    type Value = Context<Self>; /* Context data is of type Streamer */

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        ctx.data
    }

    fn get_data(&self) -> Self::Value {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
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

    fn fill_buffer(&mut self) -> &[u8] {
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
    /* casting generic N to NodeReceptor trait to access the InnerReceptor gat */
    <N as NodeReceptor>::InnerReceptor: Send + Sync + 'static{

    // with pointer we can borrow the type to prevent from moving and 
    // makes the type sizable at compile time by storing the address of 
    // none determined size of it inside the stack like str and []
    // box is sized with the size of its content allocated on the heap
    trait Test{}
    struct Neuronam{}
    let name = Neuronam{};
    impl Test for Neuronam{}
    let trait_name = &name as &dyn Test;
    struct AnotherNeuronam<T: Test, F> where F: FnOnce() -> (){
        pub data: T,
        pub new_data: F
    }
    impl<V: Test, T> AnotherNeuronam<V, T> where T: FnOnce() -> (){
        fn get_data(param: impl FnMut() -> ()) -> impl FnMut() 
            -> std::pin::Pin<Box<dyn std::future::Future<Output=String> + Send + Sync + 'static>>{
            ||{
                Box::pin(async move{
                    String::from("")
                })
            }
        }
        fn get_func() -> fn() -> String{
            fn get_name() -> String{
                String::from("")
            }
            get_name
        }
        }
    let another_name = AnotherNeuronam{data: name, new_data: ||{}};

    let pinned_boxed_future: std::pin::Pin<Box<dyn std::future::Future<Output=String>>> = 
        Box::pin(async move{
            String::from("")
        });

    let cls = |func: fn() -> String|{
        func()
    };
    fn execute() -> String{
        String::from("wildonion")
    }
    cls(execute);

    let cls = ||{};
    let casted = &cls as &dyn Fn() -> (); // casting the closure to an Fn trait
    let name = (
        |name: String| -> String{
            name
        }
    )(String::from(""));
    
    enum Packet{
        Http{header: String},
        Tcp{size: usize}, // the size of the incoming buffer
        Snowflake{id: String}
    }
    let packet = Packet::Http { header: String::from("") };
    if let Packet::Http { header } = packet{
        println!("packet header bytes => {header:}");
    }

    enum UserName{
        Age,
        Id,
        Snowflake{id: String}
    }
    let enuminstance = (Packet::Tcp{size: 0 as usize}, Packet::Http { header: String::from("http header") });
    let res = match enuminstance{
        (Packet::Tcp { size: tcpsize }, Packet::Http{ header: httpheader }) | 
        (Packet::Http{ header: httpheader }, Packet::Tcp { size: tcpsize }) => {},
        (_, Packet::Snowflake{id: sid}) => if !sid.is_empty(){},
        _ => {}
    };

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

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum ActionType{
    #[default]
    A1
} 
type Method = fn() -> i32;
fn run<'lifetime>(param: impl Fn() -> ActionType, method: &'lifetime Method)
// bounding generic Method to traits and lifetimes
where Method: Send + Sync + 'static{}
fn execute<'f, F>(param: &'f mut F) -> () 
// bounding generic F to closure, lifetimes and other traits
where F: Fn() -> ActionType + Send + Sync + 'static{}

// bounding generic to traits and lifetiems
// async trait fn run in multithread env using #[trait_variant::make(TraitNameSend: Send)]
// bounding trait method only to traits like TraitName::foo(): Send + Sync
// return trait from method using -> impl TraitName
// trait as method param like param: impl TraitName
// trait as struct field like pub data: F (where F: TraitName) or pub data: Box<dyn TraitName> 
// casting generic to trait like &N as &dyn TraitName or N as TraitName
// bounding trait gat to traits like <N as TraitName>::AssetInfo: Send + Sync
// bounding the return type of closure trait to traits like where F: FnOnce() -> R + Send + Sync + 'static
trait Interface: Send + Sync + 'static{}
struct Instance{}
impl Interface for Instance{}
impl Interface for (){}
type BoxedTrait = Box<dyn FnOnce() -> ()>;
struct Test<R, F: Send + Sync + 'static + Clone + Default> 
    where F: FnOnce() -> R + Send + Sync + 'static, 
        R: Send + Sync + 'static{
    pub data: F,
    pub another_data: BoxedTrait
}
fn trait_as_ret_and_param_type(param: &mut impl FnOnce() -> ()) -> impl FnOnce() -> (){ ||{} }
fn trait_as_ret_and_param_type1(param_instance: &mut impl Interface) -> impl FnOnce() -> (){ ||{} }
fn trait_as_ret_type(instance_type: Instance) -> impl Interface{ instance_type }
fn trait_as_ret_type_1(instance_type: Instance) -> impl Interface{ () }
fn trait_as_param_type(param: impl FnOnce() -> ()){}


// C must be send sync to be share between threads safely
impl<F: Interface + Clone, C: Send + Sync + 'static + FnOnce() -> String> Interface for UserInfo<C, F>{}
struct UserInfo<C: Send + Sync + 'static, F: Clone> where 
    F: Interface, 
    C: FnOnce() -> String{
    data: F,
    __data: C,
    _data: Box<dyn Interface>,
}
impl<F: Interface + Clone, C: Send + Sync + 'static + FnOnce() -> String> UserInfo<C, F>{
    fn set_data(cls: impl FnOnce() -> String, clstopass: C, f: F) -> impl Interface{
        Self{
            data: f,
            __data: clstopass,
            _data: Box::new(
                ()
            ),
        }
    }
} 