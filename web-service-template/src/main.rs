use actix_files::Files;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, ResponseError};
use derive_more::{Display, Error};
use handlebars::Handlebars;
use serde::Serialize;
use std::{env::var, io};

#[derive(Serialize)]
struct Compliment {
    name: &'static str,
    adjective: &'static str,
}

#[derive(Debug, Display, Error)]
struct HttpError {
    message: String,
}

impl ResponseError for HttpError {}

#[get("/")]
async fn compliment(hb: Data<Handlebars<'_>>) -> Result<HttpResponse, HttpError> {
    let compliment = Compliment {
        name: "Alice",
        adjective: "beautiful",
    };
    match hb.render("compliment", &compliment) {
        Ok(r) => Ok(HttpResponse::Ok().content_type("text/html").body(r)),
        Err(e) => Err(HttpError { message: e.desc }),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind_address = var("BIND_ADDRESS").unwrap_or_else(|_| "localhost:8080".to_string());
    let template_service = {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".html", "web/templates")
            .expect("Could not register templates directory");
        Data::new(handlebars)
    };
    let server = move || {
        App::new()
            .app_data(template_service.clone())
            .service(Files::new("/public", "web/public").show_files_listing())
            .service(compliment)
    };
    HttpServer::new(server).bind(bind_address)?.run().await
}
