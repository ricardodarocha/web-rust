use std::default;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use serde::Deserialize;

use crate::{app::AppState, models::*};

    fn default_start() -> String {
    "2000-01-01 00:00:00 +00:00".to_owned()
    }

    fn default_end() -> String {
    "2030-01-01 00:00:00 +00:00".to_owned()
    }
#[derive(Deserialize, Clone)]
struct QueryParams {
    #[serde(default="default_start")]
    start_date: String,
    #[serde(default="default_end")]
    end_date: String,
}

impl Default for QueryParams {
    fn default() -> Self { 
        
        let start_date = "2000-01-01 00:00:00 +00".to_owned();
        let end_date   = "2030-01-01 00:00:00 +00".to_owned();
        QueryParams {
            start_date,
            end_date,
        }

     }
}

#[get("/json")]
async fn get_vendas_json(pool: web::Data<AppState>, 
    // query: web::Query<QueryParams>,
    ) 
    -> impl Responder {
    
    let pool = &pool.database.conn;
    let result = RepositoryVendas::get_sales_by_period(&pool, 
        // &query.start_date, 
        // &query.end_date,
    ).await;

    match result {
        Ok(sales) => HttpResponse::Ok().json(sales),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

    #[get("/api/ping")]
    pub async fn ping() -> impl Responder {
    const MESSAGE: &str = "🟢 Running";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[get("/")]
async fn get_vendas(pool: web::Data<AppState>,
    // query: web::Query<QueryParams>,
    ) 
    -> impl Responder { 
        let pool = &pool.database.conn;
        let vendas = RepositoryVendas::get_sales_by_period(&pool, 
            // &query.start_date, 
            // &query.end_date,
        ).await;
        crate::view::vendas::view_vendas(vendas.unwrap())
    }