use rocket::response;
use rocket::serde::json::json;
use rocket::serde::Serialize;

use crate::common::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn wrap_ok<S: Serialize>(serializable: S) -> response::content::RawJson<String> {
    wrap_ok_with_warnings(serializable, &([] as [&str; 0]))
}

pub fn wrap_ok_with_warnings<S: Serialize, M: Serialize>(serializable: S, messages: &[M]) -> response::content::RawJson<String> {
    response::content::RawJson(serde_json::to_string(&json!({
        "messages": messages.iter().map(|m| {
            json!({
                "type": "warning",
                "value": m,
            })
        }).collect::<Vec<_>>(),
        "data": serializable,
    })).unwrap())
}
