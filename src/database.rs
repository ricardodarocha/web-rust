use std::time::Duration;

use log::info;
use sqlx::{FromRow, Pool, Postgres};

//Checar como implementar Entidades em DDD Domain Driven Design
#[allow(unused)]
#[derive(FromRow, serde::Deserialize)]
pub struct EntityId {
    pub id: String, //v7
}

pub trait DbConnGetter {
    type Output;
    fn get_conn(&self) -> &Self::Output;
}

#[derive(Clone)]
pub struct DbInstance {
    pub conn: Pool<Postgres>
}

impl DbInstance {
    pub async fn init() -> Self {
        Self { conn: get_database_connection().await }
    }
}

impl DbConnGetter for DbInstance {
    type Output = Pool<Postgres>;

    fn get_conn(&self) -> &Self::Output {
        &self.conn    
    }
}


#[derive(FromRow, serde::Deserialize)]
pub struct Profile {
    pub value: String,
}

pub async fn get_database_connection() -> Pool<Postgres> {
    dotenv::dotenv().ok();
    // let host = std::env::var("HOST").unwrap();
    // let database = std::env::var("DATABASE").unwrap();
    // let port = std::env::var("PORT").unwrap();
    // let user = std::env::var("USER").unwrap();
    // let password = std::env::var("PASSWORD").unwrap();
    
    // let database_url = format!(
    //     "postgres://{user}:{password}@{host}:{port}/{database}"
    // );

    
    let database_url = std::env::var("DATABASE_URL").unwrap();
    info!("{url}", url=database_url.clone());
    let conn = sqlx::postgres::PgPool::connect(&database_url)
     .await.unwrap();
    
    conn
}

/// Função wrapper para verificar se a conexão está ativa e fechar o pool se a conexão falhar.
/// Na próxima operação, o pool será reaberto automaticamente.
pub async fn autorecover(pool: &Pool<Postgres>) -> Pool<Postgres> {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            pool.clone()
        }
        Err(e) => {
            println!("{} \n A conexão foi reiniciada automaticamente", e);
            pool.close().await;
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
            pool.clone()
        }
    }
}