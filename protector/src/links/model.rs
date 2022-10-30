use crate::db;
use crate::error_handler::CustomError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "link_report"]
pub struct LinkReport {
    pub link_des: String,
    pub origin_name: String,
    pub count_confirm: i32,
    pub count_report: i32,
    pub category_id: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "link_report"]
pub struct PhoneReportImpl {
    pub id: i32,
    pub link_des: String,
    pub origin_name: String,
    pub count_confirm: i32,
    pub count_report: i32,
    pub category_id: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "link_search"]
pub struct LinkSearch {
    pub link_des: String,
    pub count_search: i32,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "link_search"]
pub struct PhoneSearchImpl {
    pub id: i32,
    pub link_des: String,
    pub count_search: i32,
    pub date_created: String,
    pub date_modified: String,
}


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "link_category"]
pub struct LinkCategory {
    pub type_link: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "link_category"]
pub struct LinkCategoryImpl {
    pub id: i32,
    pub type_link: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "link_comment"]
pub struct LinkComment {
    pub link_id: i32,
    pub name_user: String,
    pub body_comment: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "link_comment"]
pub struct LinkCommentImpl {
    pub id: i32,
    pub link_id: i32,
    pub name_user: String,
    pub body_comment: String,
    pub date_created: String,
    pub date_modified: String,
}


impl LinkReportImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let link = link_report::table.load::<LinkReportImpl>(&conn)?;
        Ok(link)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = link_report::table.filter(link_report::id.eq(id)).first(&conn)?;
        Ok(link)
    }


    pub fn create(link_report: LinkReport) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = LinkReport::from(link_report);
        let link = diesel::insert_into(link_report::table)
            .values(link)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn update(id: i32, link_report: LinkReport) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = diesel::update(link_report::table)
            .filter(link_report::id.eq(id))
            .set(link_report)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(link_report::table.filter(link_report::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl LinkReport {
    fn from(link: LinkReport) -> LinkReport {
        LinkReport {
            link_des: link.link_des,
            origin_name: link.origin_name,
            count_confirm: link.count_confirm,
            count_report: link.count_report,
            category_id : link.category_id,
            date_created: link.date_created,
            date_modified: link.date_modified
        }
    }
}



impl LinkSearchImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let link = link_search::table.load::<LinkSearchImpl>(&conn)?;
        Ok(link)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = link_search::table.filter(link_search::id.eq(id)).first(&conn)?;
        Ok(link)
    }


    pub fn create(link_search: LinkSearch) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = LinkReport::from(link_search);
        let link = diesel::insert_into(link_search::table)
            .values(link)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn update(id: i32, link_search: LinkSearch) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = diesel::update(link_search::table)
            .filter(link_search::id.eq(id))
            .set(link_search)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(link_search::table.filter(link_search::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl LinkSearch {
    fn from(link: LinkReport) -> LinkSearch {
        LinkSearch {
            link_des: link.link_des,
            count_search : link.count_search,
            date_created: link.date_created,
            date_modified: link.date_modified
        }
    }
}


impl LinkCategoryImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let link = link_search::table.load::<LinkCategoryImpl>(&conn)?;
        Ok(link)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = link_category::table.filter(link_category::id.eq(id)).first(&conn)?;
        Ok(link)
    }


    pub fn create(link_category: LinkCategory) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = LinkCategory::from(link_category);
        let link = diesel::insert_into(link_category::table)
            .values(link)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn update(id: i32, link_category: LinkCategory) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = diesel::update(link_category::table)
            .filter(link_category::id.eq(id))
            .set(link_search)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(link_category::table.filter(link_category::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl LinkCategory {
    fn from(link: LinkCategory) -> LinkCategory {
        LinkSearch {
            type_link: link.type_link,
            date_created: link.date_created,
            date_modified: link.date_modified
        }
    }
}

impl LinkCommentImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let link = link_comment::table.load::<LinkCommentImpl>(&conn)?;
        Ok(link)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = link_comment::table.filter(link_comment::id.eq(id)).first(&conn)?;
        Ok(link)
    }


    pub fn create(link_comment: LinkComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = LinkComment::from(link_comment);
        let link = diesel::insert_into(link_comment::table)
            .values(link)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn update(id: i32, link_comment: LinkComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let link = diesel::update(link_comment::table)
            .filter(link_comment::id.eq(id))
            .set(link_search)
            .get_result(&conn)?;
        Ok(link)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(link_comment::table.filter(link_comment::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl LinkComment {
    fn from(link: LinkComment) -> LinkComment {
        LinkSearch {
            link_id: link.link_id,
            name_user: link.name_user,
            body_comment: link.body_comment,
            date_created: link.date_created,
            date_modified: link.date_modified
        }
    }
}