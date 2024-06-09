use actix_web::{
    get, post,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpResponse, HttpServer, Responder,
};
use entity::prelude::Users;
use entity::users;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
struct UserInput {
    name: String,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/{id}")]
async fn get_user(path: web::Path<i64>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let db = state.conn.clone();
    let result = Users::find_by_id(id).one(&db).await.unwrap();
    if let Some(user) = result {
        HttpResponse::Found().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/user")]
async fn create_user(input: web::Json<UserInput>, state: web::Data<AppState>) -> impl Responder {
    let db = state.conn.clone();
    let name = ActiveValue::set(input.clone().name);
    let user = users::ActiveModel {
        name,
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
    HttpResponse::Ok().json(user)
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

    let conn = Database::connect(&db_url).await.unwrap();
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
}
