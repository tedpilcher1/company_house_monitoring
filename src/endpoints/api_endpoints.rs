use actix_web::{
    delete, get, put,
    web::{self},
    HttpResponse, Responder,
};
use uuid::Uuid;

use crate::endpoints::{AppState, SubscribeRequest};

use super::DateRange;

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

#[get("/company_snapshots{company_house_id}")]
async fn company_snapshots_endpoint(
    company_house_id: web::Path<String>,
    state: web::Data<AppState>,
    query: web::Query<DateRange>,
) -> impl Responder {
    let date_range = query.into_inner();

    if date_range.from_date > date_range.to_date {
        return HttpResponse::BadRequest().finish();
    }

    if let Ok(mut database) = state.database.lock() {
        return match database.get_company_snapshots(
            company_house_id.to_string(),
            date_range.from_date,
            date_range.to_date,
        ) {
            Ok(snapshots) => HttpResponse::Ok().json(snapshots),
            Err(_) => HttpResponse::NotFound().finish(),
        };
    }

    HttpResponse::InternalServerError().finish()
}
