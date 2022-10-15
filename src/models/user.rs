use serde::{Serialize, Deserialize};
// use std::io::Cursor;
// use rocket::request::Request;
// use rocket::response::{self, Response, Responder};
// use rocket::http::ContentType;

// serde参考文档：https://serde.rs/variant-attrs.html
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(serialize = "uid", deserialize = "id"))]
    pub id: i32,
    pub name: String,
}

// impl<'a> Responder<'a> for User {
//     fn respond_to(self, _: &Request) -> response::Result<'a> {
//         let json=serde_json::to_string(&self).unwrap();
//         Response::build()
//             .sized_body(Cursor::new(json))
//             .header(ContentType::new("application", "json"))  
//             .ok()
//     }
// }
