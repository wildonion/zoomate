


use crate::*;


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Env{
    pub REDIS_HOST: String,  
    pub REDIS_PORT: String,  
    pub REDIS_USERNAME: String,  
    pub REDIS_PASSWORD: String,  
    pub IO_BUFFER_SIZE: String,  
    pub FILE_SIZE: String,  
    pub HOST: String,  
    pub TCP_PORT: String,  
    pub GRPC_PORT: String,  
    pub HOOPOE_PORT: String,  
    pub SECRET_KEY: String,  
    pub ENVIRONMENT: String,  
    pub MACHINE_ID: String,  
    pub NODE_ID: String,  
    pub POSTGRES_HOST: String,  
    pub POSTGRES_PORT: String,  
    pub POSTGRES_USERNAME: String,  
    pub POSTGRES_PASSWORD: String,  
    pub POSTGRES_ENGINE: String,  
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
                HOST: std::env::var("HOST").unwrap(),
                TCP_PORT: std::env::var("TCP_PORT").unwrap(),
                GRPC_PORT: std::env::var("GRPC_PORT").unwrap(),
                HOOPOE_PORT: std::env::var("HOOPOE_PORT").unwrap(),
                SECRET_KEY: std::env::var("SECRET_KEY").unwrap(),
                ENVIRONMENT: std::env::var("ENVIRONMENT").unwrap(),
                MACHINE_ID: std::env::var("MACHINE_ID").unwrap(),
                NODE_ID: std::env::var("NODE_ID").unwrap(),
                POSTGRES_HOST: std::env::var("POSTGRES_HOST").unwrap(),
                POSTGRES_PORT: std::env::var("POSTGRES_PORT").unwrap(),
                POSTGRES_USERNAME: std::env::var("POSTGRES_USERNAME").unwrap(),
                POSTGRES_PASSWORD: std::env::var("POSTGRES_PASSWORD").unwrap(),
                POSTGRES_ENGINE: std::env::var("POSTGRES_ENGINE").unwrap(),
            }
        };

        ctx
        
    }

}