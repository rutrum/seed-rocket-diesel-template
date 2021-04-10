#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::json::Json;

use api::db;

#[get("/")]
fn test() -> String {
    "It works!".into()
}

mod student {
    use super::*;
    use seed_rocket_diesel_template::student::*;

    #[post("/student", data = "<item>")]
    pub fn create(item: Json<NewStudent>) -> Status { // shouldn't be status, but the new student
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::student::insert(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
        
    }

    #[get("/student")]
    pub fn read() -> Json<Vec<Student>> {
        let conn = db::create_connection();
        let items = db::student::select(&conn).unwrap_or_default(); // don't do this
        Json(items)
    }

    #[put("/student", data = "<item>")]
    pub fn update(item: Json<Student>) -> Status { 
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::student::update(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/student", data = "<item>")]
    pub fn delete(item: Json<Student>) -> Status { 
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::student::delete(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            test,
            student::create,
            student::read,
            student::update,
            student::delete,
        ])
        .launch();
}
