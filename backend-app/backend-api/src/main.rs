mod models;
mod handlers;
mod db;
mod sync_service;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/crm".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    println!("🚀 Server starting on http://0.0.0.0:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(handlers::health_check)
                    // New unified sync endpoint
                    .route("/sync", web::post().to(sync_service::sync_all))
                    // Legacy endpoints (kept for compatibility)
                    .service(handlers::sync_user)
                    .service(handlers::sync_company)
                    .service(handlers::sync_customer)
                    .service(handlers::sync_document)
                    .service(handlers::sync_proposal)
                    .service(handlers::sync_proposal_product)
                    .service(handlers::get_users)
                    .service(handlers::get_companies)
                    .service(handlers::get_customers)
                    .service(handlers::get_documents)
                    .service(handlers::get_proposals)
                    .service(handlers::get_proposal_products)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

