use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use std::{env, io};

pub mod data;
pub mod routes;
pub mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = env::var("ADDRESS").unwrap_or("localhost".to_string());
    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    println!("Starting server at http://{}:{}/", host, port);

    HttpServer::new(|| {
        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(routes::solver::solve)
            .service(routes::solver::solve_once)
            .service(routes::solver::filled);

        app = app.service(
            fs::Files::new("/", "./wasm/static/")
                .prefer_utf8(true)
                .show_files_listing()
                .index_file("index.html"),
        );

        // If environment is wasm, serve static files
        if env::var("WASM").is_ok() {
        } else {
        }

        return app;
    })
    .bind((host, port))?
    .workers(4)
    .run()
    .await
}
