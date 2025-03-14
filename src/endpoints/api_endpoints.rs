use actix_web::{put, web, HttpResponse, Responder};

use crate::endpoints::{AppState, SubscribeRequest};

#[put("/subscribe/{company_house_id}")]
async fn subscribe_endpoint(
    company_house_id: web::Path<String>,
    state: web::Data<AppState>,
    request_body: web::Json<SubscribeRequest>,
) -> impl Responder {
    let subscribe_request = request_body.into_inner();
    if let Ok(mut database) = state.database.lock() {
        let res = match database.create_subscription(
            company_house_id.to_string(),
            subscribe_request.notable_changes,
            subscribe_request.url,
        ) {
            Ok(subscription_id) => HttpResponse::Ok().json(subscription_id),
            Err(_) => HttpResponse::InternalServerError().finish(),
        };
        return res;
    }
    HttpResponse::InternalServerError().finish()
}
