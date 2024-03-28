


// https://github.com/YouWhoClub/wallet-backend/blob/main/core/panel/events/publishers/user.rs

// use dynamic dispatching and object safe, dynamic typing and polymorphism
// store notifs in spacetimedb  


use crate::*;

#[derive(Serialize, Deserialize)]
pub enum ActionType{

}

#[derive(Serialize, Deserialize)]
pub struct UserNotif{
    user_info: UserInfo,
    notifs: Vec<NotifData>
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo{
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NotifData{
    pub action_data: serde_json::Value, // any data
    pub actioner_info: UserInfo,
    pub action_type: ActionType, // type event
    pub fired_at: Option<i64>, 
    pub is_seen: bool,
}

// make the trait Sendable cause it has async methods
#[trait_variant::make(UserNotifExtSend: Send)] 
pub trait UserNotifExt{
    async fn get_notifs(&self) -> Vec<NotifData>;
    fn set_notifs(&mut self) -> Vec<NotifData>;
    fn extend_notifs(&mut self) -> Vec<NotifData>;
}

pub async fn emit(){

    // publish a NotifData event to redis channel
    // ...

}