pub mod vendas {
    

use crate::controller::*;

    // Define as rotas para o controlador de autenticação
    pub fn routes(cfg: &mut crate::web::ServiceConfig) {
    cfg.service(
        crate::web::scope("/vendas") 
            .service(get_vendas_json)
            .service(get_vendas)
        );
    }
}