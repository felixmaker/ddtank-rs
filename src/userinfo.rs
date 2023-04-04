use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub server: String,
    pub nickname: String,
    pub strategy: String,
    pub date: Option<SystemTime>,
}

// impl From<UserInfo> for sciter::Value {
//     fn from(u: UserInfo) -> Self {
//         let userinfo = serde_json::to_string(&u).unwrap();
//         let user = sciter::value::Value::parse(&userinfo).unwrap();
//         user
//     }
// }

// impl sciter::FromValue for UserInfo {
//     fn from_value(v: &sciter::Value) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         let json_string = v.clone().into_string();
//         let userinfo = serde_json::from_str::<UserInfo>(&json_string);
//         userinfo.ok()
//     }
// }
