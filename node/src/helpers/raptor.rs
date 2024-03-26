


/* 
    https://github.com/wildonion/courtokm/
*/

use crate::*;


#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Raptor{

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

    fn get_room<'valid>(&mut self, cls: impl FnOnce(String) -> String) -> Self::Room<'valid> {

        // no two closures, even if identical, have the same type since 
        // they're traits and traits are not sized
        // consider boxing your closure and/or using it as a trait object
        // trait object is an struct instance that implements the trait 
        // and we canll trait method on it, also boxing makes the type
        // sizable cause box stores on the heap, in our case they must be
        // as trait object cause they can be sized since we're calling them
        // on an struct instance
        // let c1 = Box::new(||{});
        // let mut closure = &mut c1;
        // closure = &mut Box::new(||{}); // it can't be mutated cause

        fn get_name() -> String{ String::from("") }
        let callback = |func: fn() -> String|{
            func();
        };
        callback(get_name);

        type Closure = Box<dyn FnOnce() -> ()>;
        fn get_cls(cls: Closure) -> (){ () }
        fn get_cls1(cls: impl FnOnce() -> ()) -> (){ () }
        // or
        (   
            // a closure that takes a closure
            |param: Box<dyn FnOnce() -> ()>|{
                ()
            }
        )(Box::new(||{}));
        
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