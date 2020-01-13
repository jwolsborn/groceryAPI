use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct TheList {
    items: Vec<String>,
}

async fn add(req: HttpRequest, data: web::Data<Mutex<TheList>>) -> impl Responder {
    let mut data = data.lock().unwrap();
    let item = req.match_info().get("item").unwrap();
    data.items.push(item.to_string());
    format!("Item {} added", &item)
}

async fn get(data: web::Data<Mutex<TheList>>) -> impl Responder {
    let data = data.lock().unwrap();
    web::Json(data.items.clone())
}

async fn remove(req: HttpRequest, data: web::Data<Mutex<TheList>>) -> impl Responder {
    let mut data = data.lock().unwrap();
    let item = req.match_info().get("item").unwrap();
    data.items.retain(|x| x != item);
    format!("Item {} removed", &item) 
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let groceries = web::Data::new(Mutex::new(TheList{items: Vec::new()}));
    
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
