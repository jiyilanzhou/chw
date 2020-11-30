
use crate::model;
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool,Client};
use crate::db;
use std::io::ErrorKind::Other;

async fn status() -> impl Responder {
    // "{\"status\":\"up\"}"
    web::HttpResponse::Ok()
        .json(model::Status{ status: "OK".to_string() })
}

pub async fn query_lists(db_pool:web::Data<Pool>)->impl Responder{
    let client:Client = db_pool.get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_lists(&client);

    match result{
        Ok(list)=>HttpResponse::Ok().json(lists),
        Err(_)=>HttpResponse::InternalServerError().into()
    }

}

pub async fn query_items(db_pool:web::Data<Pool>,path:web::Path<(i32,)>)->impl Responder{
    let client:Client = db_pool.get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_items(&client,path.0);

    match result{
        Ok(items)=>HttpResponse::Ok().json(items),
        Err(_)=>HttpResponse::InternalServerError().into()
    }

}

pub async fn create_list(db_pool:web::Data<Pool>,json:web::Json<model::CreateList>)->impl Responder{
    let client:Client = db_pool.get()
        .await
        .expect("Error connecting to the database");
    let result = db::insert_list(&client,json.title.clone());

    match result{
        Ok(list)=>HttpResponse::Ok().json(list),
        Err(_)=>HttpResponse::InternalServerError().into()
    }

}

pub async fn check_item(db_pool:web::Data<Pool>,path:web::Path<(i32,i32)>)->impl Responder{
    let client:Client = db_pool.get()
        .await
        .expect("Error connecting to the database");
    let result = db::inspect_item(&client,path.0.into(),path.1);

    match result{
        Ok(())=>HttpResponse::Ok().json(model::ResultResponse{ success: true }),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(model::ResultResponse{ success: false }),
        Err(_)=>HttpResponse::InternalServerError().into()
    }

}