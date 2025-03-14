use actix_web::{put, web, HttpResponse, Responder};

use crate::endpoints::{AppState, SubscribeRequest};

#[put("/subscribe/{company_house_id}")]
async fn subscribe_endpoint(
    company_house_id: web::Path<String>,
    state: web::Data<AppState>,
    request_body: web::Json<SubscribeRequest>,
) -> impl Responder {
    let subscribe_request = request_body.into_inner();
    state
        .database
        .lock()
        .map(|mut database| {
            database
                .create_subscription(
                    company_house_id.to_string(),
                    subscribe_request.notable_changes,
                    subscribe_request.url,
                )
                .map_or_else(
                    |_| HttpResponse::InternalServerError().finish(),
                    |_| HttpResponse::Ok().finish(),
                )
        })
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}
