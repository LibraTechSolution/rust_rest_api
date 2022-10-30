use crate::db;
use crate::error_handler::CustomError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::phone_report;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "phone_report"]
pub struct PhoneReport {
    pub phone_number: String,
    pub origin_name: String,
    pub count_confirm: i32,
    pub count_report: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "phone_report"]
pub struct PhoneReportImpl {
    pub id: i32,
    pub phone_number: String,
    pub origin_name: String,
    pub count_confirm: i32,
    pub count_report: i32,
    pub date_created: String,
    pub date_modified: String,
}

//
// #[derive(Serialize, Deserialize, AsChangeset, Insertable)]
// #[table_name = "phone_search"]
// pub struct PhoneSearch {
//     pub phone_number: String,
//     pub count_search: i32,
//     pub date_created: String,
//     pub date_modified: String,
// }
//
// #[derive(Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "phone_search"]
// pub struct PhoneSearchImpl {
//     pub id: i32,
//     pub phone_number: String,
//     pub count_search: i32,
//     pub date_created: String,
//     pub date_modified: String,
// }
//
// #[derive(Serialize, Deserialize, AsChangeset, Insertable)]
// #[table_name = "phone_comment"]
// pub struct PhoneComment {
//     pub phone_id: i32,
//     pub name_user: String,
//     pub body_comment: String,
//     pub date_created: String,
//     pub date_modified: String,
// }
//
// #[derive(Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "phone_comment"]
// pub struct PhoneCommentImpl {
//     pub id: i32,
//     pub phone_id: i32,
//     pub name_user: String,
//     pub body_comment: String,
//     pub date_created: String,
//     pub date_modified: String,
// }
//

impl PhoneReportImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let phone = phone_report::table.load::<PhoneReportImpl>(&conn)?;
        Ok(phone)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = phone_report::table.filter(phone_report::id.eq(id)).first(&conn)?;
        Ok(phone)
    }


    pub fn create(phone: PhoneReport) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = PhoneReport::from(phone);
        let phone = diesel::insert_into(phone_report::table)
            .values(phone)
            .get_result(&conn)?;
        Ok(phone)
    }

    pub fn update(id: i32, phone_report: PhoneReport) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = diesel::update(phone_report::table)
            .filter(phone_report::id.eq(id))
            .set(phone_report)
            .get_result(&conn)?;
        Ok(phone)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(phone_report::table.filter(phone_report::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl PhoneReport {
    fn from(phone: PhoneReport) -> PhoneReport {
        PhoneReport {
            phone_number: phone.phone_number,
            origin_name: phone.origin_name,
            count_confirm: phone.count_confirm,
            count_report: phone.count_report,
            date_created: phone.date_created,
            date_modified: phone.date_modified
        }
    }
}