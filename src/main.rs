use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::StatusCode};
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Groceries {
    items: Vec<String>,
}

#[derive(Serialize)]
struct JsonRes {
	data: Vec<String>,
	message: String,
}

async fn add(req: HttpRequest, data: web::Data<Mutex<Groceries>>) -> impl Responder {
    let mut data = match data.lock(){
		Err(_) => return web::Json(format!("Data is locked")).with_status(StatusCode::BAD_REQUEST),
		Ok(data) => data
	};
    let item = match req.match_info().get("item"){
		None => return web::Json(format!("Unable to retrieve request")).with_status(StatusCode::BAD_REQUEST),
		Some(item) => item
	};
    data.items.push(item.to_string());
    web::Json(format!("Item {} added", &item)).with_status(StatusCode::OK)
}

async fn get(data: web::Data<Mutex<Groceries>>) -> impl Responder {
    let data = match data.lock(){
   		Err(_) => return web::Json(JsonRes{data: Vec::new(), message:"Data is locked".to_string()}).with_status(StatusCode::BAD_REQUEST),
   		Ok(data) => data
   };

	web::Json(JsonRes{data:data.items.clone(), message: "Success".to_string()}).with_status(StatusCode::OK)
}

async fn remove(req: HttpRequest, data: web::Data<Mutex<Groceries>>) -> impl Responder {
    let mut data = match data.lock(){
    	Err(_) => return web::Json(format!("Data is locked")).with_status(StatusCode::BAD_REQUEST),
    	Ok(data) => data
	};
	let item = match req.match_info().get("item"){
		None => return web::Json(format!("Unable to retrieve request")).with_status(StatusCode::BAD_REQUEST),
		Some(item) => item
	};
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
