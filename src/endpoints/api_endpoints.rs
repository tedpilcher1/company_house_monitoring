use actix_web::{delete, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::endpoints::{AppState, SubscribeRequest};

#[put("/subscribe/{company_house_id}")]
async fn subscribe_endpoint(
    company_house_id: web::Path<String>,
    state: web::Data<AppState>,
    request_body: web::Json<SubscribeRequest>,
) -> impl Responder {
    let subscribe_request = request_body.into_inner();
    if let Ok(mut database) = state.database.lock() {
        return match database.create_subscription(
            company_house_id.to_string(),
            subscribe_request.notable_changes,
            subscribe_request.url,
        ) {
            Ok(subscription_id) => HttpResponse::Ok().json(subscription_id),
            Err(_) => HttpResponse::InternalServerError().finish(),
        };
    }
    HttpResponse::InternalServerError().finish()
}

#[delete("/unsubscribe/{subscription_id}")]
async fn unsubscribe_endpoint(
    subscription_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Ok(mut database) = state.database.lock() {
        return match database.delete_subscription(*subscription_id) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::NotFound().finish(),
        };
    }
    HttpResponse::InternalServerError().finish()
}
