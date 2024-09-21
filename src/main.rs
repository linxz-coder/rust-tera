use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use std::sync::Mutex;
use serde::Serialize;  // 添加这行来导入 Serialize

struct AppState {
    tera: Mutex<Tera>,
}

#[derive(Serialize)]
struct UserInfo {
    name: String,
    age: u32,
}

#[get("/")]
async fn index(data: actix_web::web::Data<AppState>) -> impl Responder {
    let mut tera = data.tera.lock().unwrap();
    tera.full_reload().unwrap();

    let mut context = tera::Context::new();
    
    let user_infos = vec![
        UserInfo { name: "张三".to_string(), age: 18 },
        UserInfo { name: "李四".to_string(), age: 20 },
        UserInfo { name: "王五".to_string(), age: 22 },
    ];

    context.insert("name", "世界");
    context.insert("user_infos", &user_infos);

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