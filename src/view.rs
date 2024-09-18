pub mod vendas {
    use actix_web::Responder;
    use minijinja::context;
    // use serde_json::json;

    use crate::models::ResumoVendas;

    pub fn view_vendas(vendas: Vec<ResumoVendas>) -> impl Responder {
        // let vendas = json!([{"cliente_nome": "ACME Corp", "total_vendido": 10000}]);

        crate::render::render_minijinja(
            "view_vendas.html", 
            context!(vendas),
        )
    }
}