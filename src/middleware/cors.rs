use actix_cors::Cors;
use actix_web::http::{header, Method};

pub fn cors() -> Cors {
    Cors::default()
        // .allowed_origin("http://127.0.0.1:3000")
        // .supports_credentials()
        .allow_any_origin()
        .allowed_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allowed_headers([header::CONTENT_TYPE])
    // .max_age(86_400)
}
