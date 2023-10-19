


use actix::prelude::*;
use crate::*;


#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Raptor{

}

impl Actor for Raptor{
    type Context = Context<Raptor>;
}