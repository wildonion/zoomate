



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
        tokio::sync::mpsc::channel::<tokio::sync::Mutex<Fork>>();

    let (rwlock_fork_request_sender, mut rwlock_fork_request_receiver) = 
        tokio::sync::mpsc::channel::<tokio::sync::RwLock<Fork>>();
  
}


