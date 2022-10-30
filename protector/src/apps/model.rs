use crate::db;
use crate::error_handler::CustomError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "app_report"]
pub struct AppReport {
    pub app_name: String,
    pub app_image: String,
    pub app_bundle_android: String,
    pub app_bundle_ios: String,
    pub app_des : String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "app_report"]
pub struct AppReportImpl {
    pub id: i32,
    pub app_name: String,
    pub app_image: String,
    pub app_bundle_android: String,
    pub app_bundle_ios: String,
    pub app_des : String,
    pub date_created: String,
    pub date_modified: String,
}



#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "app_comment"]
pub struct AppComment {
    pub app_id: i32,
    pub name_user: String,
    pub body_comment: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "app_comment"]
pub struct AppCommentImpl {
    pub id: i32,
    pub app_id: i32,
    pub name_user: String,
    pub body_comment: String,
    pub date_created: String,
    pub date_modified: String,
}

impl AppReportImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let app = app_report::table.load::<AppReportImpl>(&conn)?;
        Ok(app)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let app = app_report::table.filter(app_report::id.eq(id)).first(&conn)?;
        Ok(app)
    }


    pub fn create(app: AppReport) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let app = AppReport::from(app);
        let app = diesel::insert_into(app_report::table)
            .values(app)
            .get_result(&conn)?;
        Ok(app)
    }

    pub fn update(id: i32, app_report: AppReport) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let app = diesel::update(app_report::table)
            .filter(app_report::id.eq(id))
            .set(app_report)
            .get_result(&conn)?;
        Ok(app)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(app_report::table.filter(app_report::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}


impl AppReport {
    fn from(app: AppReport) -> AppReport {
        AppReport {
            app_name: app.app_name,
            app_image: app.app_image,
            app_bundle_android: app.app_bundle_android,
            app_bundle_ios: app.app_bundle_ios,
            app_des: app.app_des,
            date_created: app.date_created,
            date_modified: app.date_modified
        }
    }
}

impl AppCommentImpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let app = app_comment::table.load::<AppCommentImpl>(&conn)?;
        Ok(app)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let app = app_comment::table.filter(app_comment::id.eq(id)).first(&conn)?;
        Ok(app)
    }


    pub fn create(app: AppComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let app = AppReport::from(app);
        let app = diesel::insert_into(app_comment::table)
            .values(app)
            .get_result(&conn)?;
        Ok(app)
    }

    pub fn update(id: i32, app_report: AppComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let app = diesel::update(app_comment::table)
            .filter(app_comment::id.eq(id))
            .set(app_report)
            .get_result(&conn)?;
        Ok(app)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(app_comment::table.filter(app_comment::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}


impl AppComment {
    fn from(app: AppComment) -> AppComment {
        AppComment {
            app_id: app.app_id,
            name_user: app.name_user,
            body_comment: app.body_comment,
            date_created: app.date_created,
            date_modified: app.date_modified
        }
    }
}