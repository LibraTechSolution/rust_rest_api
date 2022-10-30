use rocket::serde::json::serde_json::json;
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::phones::PhoneReport;


#[get("/phoneReports")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let phone = PhoneReport::find_all()?;
    Ok(HttpResponse::Ok().json(phone))
}

#[get("/phoneReports/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneReport::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[post("/phoneReports")]
async fn create(phone_input: web::Json<PhoneReport>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneReport::create(phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[put("/phoneReports/{id}")]
async fn update(
    id: web::Path<i32>,
    phone_input: web::Json<PhoneReport>,
) -> Result<HttpResponse, CustomError> {
    let phone = PhoneReport::update(id.into_inner(), phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[delete("/phoneReports/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_phone = PhoneReport::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_phone })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
