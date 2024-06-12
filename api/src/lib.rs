use std::env;

use actix_web::{
    get,
    middleware::{Logger, NormalizePath, TrailingSlash},
    post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};
use service::query::Query;
use service::{inputs::UserInput, mutation::Mutation};

use crate::ws::Ws;

mod event;
mod ws;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/{id}")]
async fn get_user(path: web::Path<i64>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let db = &data.conn;
    let result = Query::find_user_by_id(db, id)
        .await
        .expect("Something went wrong.");
    if let Some(user) = result {
        HttpResponse::Found().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/user")]
async fn create_user(form: web::Json<UserInput>, data: web::Data<AppState>) -> impl Responder {
    let db = &data.conn;
    let user = Mutation::create_user(db, form.to_owned())
        .await
        .expect("Couldn't insert user");
    HttpResponse::Ok().json(user.id.as_ref())
}

#[get("/updates/{id}")]
async fn updater(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    actix_web_actors::ws::start(Ws, &req, stream)
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
pub async fn start() -> std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(state.clone()))
            .configure(init)
    })
    .bind(server_url)?
    .run()
    .await?;
    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root);
    cfg.service(get_user);
    cfg.service(create_user);
    cfg.service(updater);
}
