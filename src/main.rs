#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod models;
use rocket_contrib::json::Json;
use mysql::*;
use mysql::prelude::*;

// mod lib;
// use lib::Json;

#[get("/")]
fn hello() -> String {
    format!("Hello, Hedui!")
}

#[get("/users?<page>&<pagesize>")]
fn list_user(page: Option<i32>, pagesize: Option<i32>) -> String {
    let page = page.unwrap_or(1);
    let pagesize = pagesize.unwrap_or(10);

    format!("list_user, page = {}, pagesize = {}", page, pagesize)
}

#[get("/users/<id>")]
fn get_user(id: i32) -> Json<models::User> {
    Json(models::User{id: id, name: String::from("hedui")})
}

#[post("/users", format="json", data="<user>")]
fn create_user(user: Json<models::User>) -> Json<models::User> {
    user
}

fn main() {
    let dsn = "mysql://root:czc19990402.@gz-cynosdbmysql-grp-rv4cxsj7.sql.tencentcdb.com:27145/helm";
    let pool = Pool::new(dsn).unwrap();
    let ret: Option<(i32, String)> = pool.get_conn().unwrap().
    query_first("select id, name from repository  order by created_at limit 1").unwrap();
    println!("{:?}", ret.unwrap().0);

    rocket::ignite().mount("/", routes![hello, list_user, get_user, create_user]).launch();
}