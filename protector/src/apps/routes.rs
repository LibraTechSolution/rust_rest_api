use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use crate::phones::{AppReport, AppReportImpl};
use serde_json::json;


#[get("/appReports")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let phone = AppReportImpl::find_all()?;
    Ok(HttpResponse::Ok().json(phone))
}

#[get("/appReports/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let phone = AppReportImpl::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[post("/appReports")]
async fn create(app_input: web::Json<AppReport>) -> Result<HttpResponse, CustomError> {
    let phone = AppReportImpl::create(app_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[put("/appReports/{id}")]
async fn update(
    id: web::Path<i32>,
    app_input: web::Json<AppReport>,
) -> Result<HttpResponse, CustomError> {
    let phone = AppReportImpl::update(id.into_inner(), app_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[delete("/phoneReports/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_phone = AppReportImpl::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_app })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}