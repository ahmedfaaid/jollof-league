#[get("/health")]
pub fn health_check() -> &'static str {
    "Health check successful!"
}
