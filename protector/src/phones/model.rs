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


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "phone_search"]
pub struct PhoneSearch {
    pub phone_number: String,
    pub count_search: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "phone_search"]
pub struct PhoneSearchImpl {
    pub id: i32,
    pub phone_number: String,
    pub count_search: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "phone_comment"]
pub struct PhoneComment {
    pub phone_id: i32,
    pub name_user: String,
    pub body_comment: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "phone_comment"]
pub struct PhoneCommentImpl {
    pub id: i32,
    pub phone_id: i32,
    pub name_user: String,
    pub body_comment: String,
    pub date_created: String,
    pub date_modified: String,
}


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

impl PhoneSearchImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let phone = phone_search::table.load::<PhoneSearchImpl>(&conn)?;
        Ok(phone)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = phone_search::table.filter(phone_search::id.eq(id)).first(&conn)?;
        Ok(phone)
    }


    pub fn create(phone: PhoneSearch) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = PhoneSearch::from(phone);
        let phone = diesel::insert_into(phone_search::table)
            .values(phone)
            .get_result(&conn)?;
        Ok(phone)
    }

    pub fn update(id: i32, phone_search: PhoneSearch) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = diesel::update(phone_search::table)
            .filter(phone_search::id.eq(id))
            .set(phone_report)
            .get_result(&conn)?;
        Ok(phone)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(phone_search::table.filter(phone_search::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl PhoneSearch {
    fn from(phone: PhoneSearch) -> PhoneSearch {
        PhoneSearch {
            phone_number: phone.phone_number,
            count_search: phone.count_search,
            date_created: phone.date_created,
            date_modified: phone.date_modified
        }
    }
}

impl PhoneCommentImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let phone = phone_comment::table.load::<PhoneCommentImpl>(&conn)?;
        Ok(phone)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = phone_comment::table.filter(phone_comment::id.eq(id)).first(&conn)?;
        Ok(phone)
    }


    pub fn create(phone: PhoneComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = PhoneComment::from(phone);
        let phone = diesel::insert_into(phone_comment::table)
            .values(phone)
            .get_result(&conn)?;
        Ok(phone)
    }

    pub fn update(id: i32, phone_comment: PhoneComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let phone = diesel::update(phone_comment::table)
            .filter(phone_comment::id.eq(id))
            .set(phone_report)
            .get_result(&conn)?;
        Ok(phone)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(phone_comment::table.filter(phone_comment::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}


impl PhoneComment {
    fn from(phone: PhoneComment) -> PhoneComment {
        PhoneComment {
            phone_id: phone.phone_id,
            name_user: phone.name_user,
            body_comment: phone.body_comment,
            date_created: phone.date_created,
            date_modified: phone.date_modified
        }
    }
}