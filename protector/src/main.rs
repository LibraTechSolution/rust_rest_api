mod phone;
mod connection;
mod link;
mod application;
mod base_reponse;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use diesel::{prelude::*, table, Insertable, Queryable};
use rocket::{fairing::AdHoc, serde::json::Json, State};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

table! {
    phone_report (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        count_report -> Int4,
        date_modified -> Varchar,
        date_created -> Varchar,
    }
}


#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/config")]
fn custom(config: &State<Config>) -> String {
    format!("Hello, {}! You are {} years old.", config.name, config.age)
}

#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, Insertable)]
#[table_name = "phone_report"]
struct PhoneReport {
    id: i32,
    title: String,
    body: String,
    count_report: i32,
    date_modified: String,
    date_created: String,
}

// #[derive(Serialize, Deserialize, Clone, Queryable, Debug, Insertable)]
// #[table_name = "phone_search"]
// struct PhoneSearch {
//     id: i64,
//     title: String,
//     body: String,
//     count_search: i32,
//     date_modified: String,
//     date_created: String,
// }
//
// #[derive(Serialize, Deserialize, Clone, Queryable, Debug, Insertable)]
// #[table_name = "phone_comment"]
// struct PhoneComment {
//     id: i64,
//     id_link: i64,
//     body: String,
//     count_search: i32,
//     name_user: String,
//     title_comment: String,
//     date_modified: String,
//     date_created: String,
// }


#[get("/")]
async fn get_phone_report(connection: Db) -> Json<Vec<PhoneReport>> {
    connection
        .run(|c| phone_report::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[post("/", data = "<phone_report>")]
async fn create_phone_report(connection: Db, phone_report: Json<PhoneReport>) -> Json<PhoneReport> {
    connection
        .run(move |c| {
            diesel::insert_into(phone_report::table)
                .values(&phone_report.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("Error")
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket
        .attach(Db::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, custom])
        .mount(
            "/phone",
            routes![
                get_phone_report,
                create_phone_report,
            ],
        )
}