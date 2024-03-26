



/*


⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
  jobq, mutex and rwlock from scratch using Dining_philosophers_problem
⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

building something jobq like mpsc, mutex and rwlock from scratch by solving the Dining_philosophers_problem 
algo using semaphores reference counting to avoid deadlocks and race conditions

https://en.wikipedia.org/wiki/Dining_philosophers_problem
https://google.github.io/comprehensive-rust/exercises/concurrency/dining-philosophers.html

  TOKIO MULTITHREADING JOBQ CHANNLE ALGORITHMS
  
    mpsc: multi-producer, single-consumer channel. Many values can be sent from different parts of the app thus the sender is cloneable
    oneshot: single-producer, single consumer channel. A single value can be sent.
    broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
    watch: single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.

    -> jobq channel to send async tasks or jobs or closures between multiple threads in a threadpool 
    -> mutex and rwlock to mutate data without having race conditions and deadlocks


Mutex (Mutual Exclusion): Rust's std::sync::Mutex provides mutual exclusion, meaning that at any given time, 
at most one thread can access the shared data inside the mutex. Relating to the dining philosophers, if each 
fork is represented as a mutex, only one philosopher can hold a fork at a time, ensuring mutual exclusion. 
Using mutexes requires careful design to avoid deadlocks.

RwLock (Read-Write Lock): std::sync::RwLock allows multiple readers or one writer to access the shared data 
simultaneously. If philosophers were allowed to either "read" (observe) or "write" (use) a fork, then multiple 
philosophers could simultaneously "read" a fork, but only one could "write" to it. In this analogy, the RwLock 
provides more flexible access than a mutex, but still ensures that data is not concurrently modified 
by multiple entities.

mpsc (Multiple Producer, Single Consumer) jobq: Rust's std::sync::mpsc provides channels for sending messages from 
multiple producers to a single consumer. While this might not have a direct mapping to the traditional dining 
philosophers problem, you can imagine an extension where philosophers send requests for forks (producers) to a 
mediator (consumer), who decides who gets which forks. The mediator ensures that no two philosophers hold the 
same fork, avoiding deadlock.


based on the fact that multiple immutable pointers and one mutable pointer can't be in a same scope at a same time 
means only one mutable and multiple immutable pointers in each scope is allowed we have the following rules:
since everything in rust must be initiated in main function thus there is not concept of global shared data structure, 
we can have static and constant but we can't change their contents because doing this is not thread safe and rust won't 
allow in the first place obviously we can have a mutexed static data which is safe to be mutated in other scopes and 
threads, also with mpsc jobq channel we can send the data from different parts of the app or threads (multiple producer) 
and only receive inside a single scope or thread (single consumer) it's obviouse that the sender of the channel must be 
cloneable since everything and every heap data in rust will be moved by passing into other scopes except stack data which 
implements Copy trait because rust don't have gc system and it's doing this to avoid having dangling pointer issue in 
futures. the dining philosopher problem is the algorithm used behind the mutex and rwlock to avoid deadlocks and race 
conditions during mutating the data inside threadpools likw tokio::spawn() by locking on the mutex to extract the data 
iniside of in such a way that only one writer thread can write to it but multiple threads can read the content.


*/

use crate::*;




fn init(){

    struct Fork;
    let (mutex_fork_request_sender, mut mutex_fork_request_receiver) = 
        tokio::sync::mpsc::channel::<tokio::sync::Mutex<Fork>>(1024);

    let (rwlock_fork_request_sender, mut rwlock_fork_request_receiver) = 
        tokio::sync::mpsc::channel::<tokio::sync::RwLock<Fork>>(1024);
  
}

mod sealer{

  trait Sealed{}

  pub trait PubSealed{
      fn do_me(&self);
  }

  pub struct PubStruct;

  impl Sealed for PubStruct{}

  impl PubSealed for PubStruct where PubStruct: Sealed{
      fn do_me(&self) {
          
      }
  }

}


fn exec(){

  struct AnotherPubType;
  impl sealer::PubSealed for AnotherPubType{
      fn do_me(&self) {
          
      }
  } 
}

pub async fn race_condition_avoidance(){

  /* ---------------------------------------------------------------------- */
  /* ---------------------- RACE CONDITION AVOIDANCE ---------------------- */
  /*  
      race conditions means that two threads want to mutate the data 
      at the same time, we have to use mutex so tell the other threads
      wait there is a threads that is trying to mutate this type and 
      will update you once the lock gets freed and in order to avoid blockcing 
      issues in the current thread we have to lock inside a separate thread 
      and mutate the type in there like tokio::spawn() then send it through 
      the jobq channel to the other threads for reading and future mutations
  */
  
  pub type ArcedMutexed<'lifetime> = std::sync::Arc<tokio::sync::Mutex<String>>;
  
  #[derive(Clone)]
  pub struct Data<D: Send + Sync + 'static>{
      /* we're using tokio mutex to avoid blocing issues inside the current thread since it locks asycnly */
      pub actual: D
  }
  let mut data_instance = Data::<ArcedMutexed>{
      actual: std::sync::Arc::new(
          tokio::sync::Mutex::new(
              String::from("a mutexed data")
          )
      ),
  };
  
  println!("data instance actual value before getting mutated >>> [{}]", data_instance.actual.lock().await.to_owned());
  
  /* reading from the channel is a mutable process thus receiver must be mutable */
  let (data_sender, mut data_receiver) = 
      tokio::sync::mpsc::channel::<Data<ArcedMutexed>>(1024);
  /*
      since tokio spawn takes a closure which captures the env vars 
      we have to use the cloned form of those types and pass them into
      the closure scopes so we can use them in later scopes 
  */
  let sender = data_sender.clone();
  tokio::spawn(async move{
      
      let new_string = String::from("an updated mutexed");
      /* 
          we're cloning data_instance and data_instance_cloned.actual to create a 
          longer lifetime value to use the cloned form to mutate, since by sending 
          data_instance_cloned to the channel its lifetime will be dropped and its 
          ownership will be moved because we're borroing the actual field by locking 
          on it so we can't move the data_instance_cloned into the mpsc channel using 
          the sender, in other words we can't move out of the type if it's behind a 
          shared reference we have to either pass a reference or clone the type and 
          work on the cloned form like the followings which we're cloning the actual 
          field to lock on its mutex and send the data_instance_cloned into 
          the downside of the channel
      */
      let data_instance_cloned = data_instance.clone();
      let data_instance_cloned_actual = data_instance_cloned.actual.clone();
      let mut data_string = data_instance_cloned_actual.lock().await; /* lock the mutex to mutate it */
      
      /* 
          mutating the locked mutex is done by dereferencing the guard 
          we're mutating data string inside the actual field in data_instance_cloned
          this will mutate the actual field inside data_instance_cloned 
      */
      *data_string = new_string; /* the actual field of the data_instance_cloned will be mutated too */

      if let Err(why) = sender.send(data_instance_cloned).await{
          println!("can't send because {:?}", why.to_string());
      }

  });

  /* receiving asyncly inside other threads to avoid blocking issues on heavy computations */
  tokio::spawn(async move{
      /* receving data asyncly while they're comming to the end of mpsc jobq channle */
      while let Some(data) = data_receiver.recv().await{
          
          let new_data_string = data.actual.lock().await.to_owned();
          println!("data instance actual value after getting mutated >>> [{}]", new_data_string);
  
      }
  });

}