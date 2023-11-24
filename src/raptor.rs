


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