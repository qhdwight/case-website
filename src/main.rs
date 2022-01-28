use actix_web::{App, get, HttpResponse, HttpServer, post, web};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

#[derive(Serialize)]
struct Product {
    name: String,
}

#[get("/")]
async fn index() -> HttpResponse {
    let mut ctx = Context::new();
    let html = TEMPLATES.render("base.html", &ctx)
        .expect("Failed to render template");
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
