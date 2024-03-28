


use crate::*;
use self::consts::ZoomateHttpResponse;


#[trait_variant::make(PassportSend: Send)]
pub trait Passport{

    type Request;

    async fn get_user(&mut self) -> ZoomateHttpResponse;
    async fn get_passport(&self) -> ZoomateHttpResponse;
}