use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer};
use company_house_monitoring::{
    database::client::DatabaseClient,
    endpoints::{
        api_endpoints::{company_snapshots_endpoint, subscribe_endpoint, unsubscribe_endpoint},
        AppState,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        database: Arc::new(Mutex::new(
            DatabaseClient::new().expect("Should be able to connect to db"),
        )),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(subscribe_endpoint)
            .service(unsubscribe_endpoint)
            .service(company_snapshots_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
