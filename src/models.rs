use log::info;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResumoVendas {
    cliente_nome: String,
    produto_nome: String,
    total_quantidade: i64,
    total_vendido: Decimal,
}

use sqlx::PgPool;
use sqlx::Error;

pub struct RepositoryVendas;
use crate::utils::*;

impl RepositoryVendas {
    pub async fn get_sales_by_period(pool: &PgPool,
        // start_date: &str, 
        // end_date: &str,
    ) 
    -> Result<Vec<ResumoVendas>, Error> {
        // info!("{}, {} ", start_date, end_date);
        // let (start_date, end_date) = (to_offset_date_time(start_date), to_offset_date_time(end_date));
        // let (start_date, end_date) = (to_primitive_date_time(start_date), to_primitive_date_time(end_date));
        let sales = sqlx::query_as!(
            ResumoVendas,
            r#"
            SELECT 
                c.Nome as "cliente_nome!", 
                p.Nome as "produto_nome!", 
                SUM(v.Quantidade) as "total_quantidade!", 
                SUM(v.Total) as "total_vendido!"
            FROM Vendas v
            JOIN Cliente c ON v.ClienteID = c.ClienteID
            JOIN Produto p ON v.ProdutoID = p.ProdutoID
            JOIN Data d ON v.DataID = d.DataID
            --WHERE d.Data BETWEEN $1::TIMESTAMP WITH TIME ZONE AND $2::TIMESTAMP  WITH TIME ZONE
            GROUP BY c.Nome, p.Nome
            "#,
            // start_date.unwrap(),
            // end_date.unwrap(),
        )
        .fetch_all(pool)
        .await?;

        Ok(sales)
    }
}
