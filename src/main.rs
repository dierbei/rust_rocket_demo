#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod models;
use rocket_contrib::json::Json;
// use mysql::*;
use mysql::prelude::*;

mod utils;

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
    utils::init_db(5, 15);
    let mut conn = utils::db().unwrap();
    let ret: Option<(i32, String)> = conn
        .query_first("select id, name from repository order by created_at desc limit 1 ")
        .unwrap();
    println!("{:?}", ret.unwrap());

    let users = utils::db().unwrap().query_map("select id, name from repository order by created_at desc", |(id, name)| {
        models::User{id: id, name: name}
    });
    println!("{:?}", users.unwrap());

    // 第一种方式
    // let stmt = conn.prep("insert into repository(id, name, url) values(?, ?, ?)").unwrap();
    // let ret = conn.exec_iter(stmt, (2, "test", "http://www.baidu.com")).unwrap();
    // println!("{}", ret.affected_rows());

    // 第二种方式
    // let ret = "insert into repository(id, name, url) values(?, ?, ?)".with((3, "test2", "www.baidu.com")).run(&mut conn).unwrap();
    // println!("{}", ret.affected_rows());

    let ret = "select id, name from repository order by created_at desc".map(&mut conn, |(id, name)| {
        models::User{id: id, name: name}
    }).unwrap();


    println!("{:?}", ret);

    rocket::ignite().mount("/", routes![hello, list_user, get_user, create_user]).launch();
}