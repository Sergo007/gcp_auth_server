mod access_token_refresher;
mod logging;
mod middleware;
mod result_ext;
mod token_refresher;
mod token_transformation;
use std::sync::Arc;

use actix_web::{error, get, middleware::Logger, web, App, HttpResponse, HttpServer};
use serde::Serialize;

use crate::access_token_refresher::AccessTokenRefresher;
use crate::token_refresher::TokenRefresher;

#[derive(Serialize)]
struct GetGcloudPrintIdentityTokenResponce {
    token: String,
}

#[get("/gcloud/print_identity_token")]
async fn gcloud_print_identity_token(
    req: actix_web::HttpRequest,
) -> web::Json<GetGcloudPrintIdentityTokenResponce> {
    let token = req
        .app_data::<Arc<TokenRefresher>>()
        .unwrap()
        .clone()
        .get()
        .await;

    let resp = GetGcloudPrintIdentityTokenResponce {
        token: token.unwrap(),
    };

    actix_web::web::Json(resp)
}

#[derive(Serialize)]
struct GetGcloudPrintAccessTokenResponce {
    token: String,
}

#[get("/gcloud/print_access_token")]
async fn gcloud_print_access_token(
    req: actix_web::HttpRequest,
) -> web::Json<GetGcloudPrintAccessTokenResponce> {
    let token = req
        .app_data::<Arc<AccessTokenRefresher>>()
        .unwrap()
        .clone()
        .get()
        .await;

    let resp = GetGcloudPrintAccessTokenResponce {
        token: token.unwrap(),
    };

    actix_web::web::Json(resp)
}

#[ctor::ctor]
fn init() {
    // dotenv().ok();
    logging::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("gcp_auth_server for auth from postman to GCP services (Cloud Run, Fhir server)");

    println!("Add this code to your Postman -> 'Pre-request Script' tab");
    println!("for any Cloud Run: gcloud auth print-identity-token");
    let s = r###"
pm.sendRequest("http://localhost:9090/gcloud/print_identity_token", function (err, response) {
    if (!err) {
        let token = response.json().token;
        pm.variables.set("access_token", token);
        pm.request.headers.add("Authorization: Bearer " + token);
    }
});
"###;
    println!("{}", s);
    println!("for any Fhir server: gcloud auth print-access-token");
    let s = r###"
pm.sendRequest("http://localhost:9090/gcloud/print_access_token", function (err, response) {
    if (!err) {
        let token = response.json().token;
        pm.variables.set("access_token", token);
        pm.request.headers.add("Authorization: Bearer " + token);
    }
});
"###;
    println!("{}", s);
    let token_refresher = TokenRefresher::arc_new();
    token_refresher.survey().await;
    let access_token_refresher = AccessTokenRefresher::arc_new();
    access_token_refresher.survey().await;
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(1024 * 1024 * 10)
            .error_handler(|err, _req| {
                // create custom error response
                println!("{}", err.to_string());
                error::InternalError::from_response(
                    err.to_string(),
                    HttpResponse::BadRequest().body(err.to_string()),
                )
                .into()
            });
        App::new()
            .wrap(Logger::default())
            .app_data(json_config)
            .app_data(token_refresher.clone())
            .app_data(access_token_refresher.clone())
            .wrap(middleware::cors::cors())
            .service(gcloud_print_identity_token)
            .service(gcloud_print_access_token)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
