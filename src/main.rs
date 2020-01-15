use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::StatusCode};
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Groceries {
    items: Vec<String>,
}

async fn add(req: HttpRequest, data: web::Data<Mutex<Groceries>>) -> impl Responder {
    let mut data = data.lock();
    let item = req.match_info().get("item");
    data.items.push(item.to_string());
    web::Json(format!("Item {} added", &item)).with_status(StatusCode::OK)
}

async fn get(data: web::Data<Mutex<Groceries>>) -> impl Responder {
    let data = data.lock();
    web::Json(data.items.clone()).with_status(StatusCode::OK)
}

async fn remove(req: HttpRequest, data: web::Data<Mutex<Groceries>>) -> impl Responder {
    let mut data = data.lock();
    let item = req.match_info().get("item");
    data.items.retain(|x| x != item);
    web::Json(format!("Item {} removed", &item)).with_status(StatusCode::OK) 
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let groceries = web::Data::new(Mutex::new(Groceries{items: Vec::new()}));
    
    HttpServer::new(move || {
        App::new()
            .app_data(groceries.clone())
            .route("/", web::get().to(get))
            .route("/{item}", web::post().to(add))
            .route("/{item}", web::put().to(remove))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
