


use crate::*;


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Env{
    pub REDIS_HOST: String,  
    pub REDIS_PORT: String,  
    pub REDIS_USERNAME: String,  
    pub REDIS_PASSWORD: String,  
    pub IO_BUFFER_SIZE: String,  
    pub FILE_SIZE: String,  
    pub TCP_HOST: String,  
    pub TCP_PORT: String,  
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Context<C>{
    vars: C

}
pub trait EnvExt{
    
    type Context;
    fn get_vars(&self) -> Self::Context;
}

impl EnvExt for Env{

    type Context = Context<Self>;

    fn get_vars(&self) -> Self::Context {
        
        let ctx = Context::<Env>{
            vars: Env{
                REDIS_HOST: std::env::var("REDIS_HOST").unwrap(),
                REDIS_PORT: std::env::var("REDIS_PORT").unwrap(),
                REDIS_USERNAME: std::env::var("REDIS_USERNAME").unwrap(),
                REDIS_PASSWORD: std::env::var("REDIS_PASSWORD").unwrap(),
                IO_BUFFER_SIZE: std::env::var("IO_BUFFER_SIZE").unwrap(),
                FILE_SIZE: std::env::var("FILE_SIZE").unwrap(),
                TCP_HOST: std::env::var("TCP_HOST").unwrap(),
                TCP_PORT: std::env::var("TCP_PORT").unwrap(),
            }
        };

        ctx
        
    }

}