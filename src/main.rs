use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use std::sync::Mutex;

struct AppState {
    tera: Mutex<Tera>,
}

#[get("/")]
async fn index(data: actix_web::web::Data<AppState>) -> impl Responder {
    let mut tera = data.tera.lock().unwrap();
    tera.full_reload().unwrap();

    let mut context = tera::Context::new();
    context.insert("name", "世界");

    match tera.render("hello.html", &context) {
        Ok(rendered) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(format!("Rendering error: {}", e)),
    }
}

#[get("/child")]
async fn child(data: actix_web::web::Data<AppState>) -> impl Responder {
    let mut tera = data.tera.lock().unwrap();
    tera.full_reload().unwrap();

    let mut context = tera::Context::new();
    context.insert("name", "访客");

    match tera.render("child.html", &context) {
        Ok(rendered) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(format!("Rendering error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(AppState {
                tera: Mutex::new(tera.clone()),
            }))
            .service(index)
            .service(child)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}