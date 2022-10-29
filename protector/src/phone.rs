// #[macro_use]
// extern crate rocket;
// #[macro_use]
// extern crate diesel;
//
// use diesel::{prelude::*, table, Insertable, Queryable};
// use rocket::{fairing::AdHoc, serde::json::Json, State};
// use rocket_sync_db_pools::database;
// use serde::{Deserialize, Serialize};

// phone_search(id) {
// id -> Int4,
// title -> Text,
// count_search -> Int4,
// date_modified -> Varchar,
// date_created -> Varchar
// }
//
// phone_comment(id) {
// id -> Int4,
// id_link -> Int4,
// title_comment -> Varchar,
// name_user -> Varchar,
// body -> Text,
// date_modified -> Varchar,
// date_created -> Varchar
// }