use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: &'static str,
}

#[get("/health")]
pub fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse { status: "OK" })
}
