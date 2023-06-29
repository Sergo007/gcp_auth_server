mod logging;
mod middleware;
use std::process::Command;

use actix_web::{error, get, middleware::Logger, web, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct GetGcloudPrintIdentityTokenResponce {
    token: String,
}

#[get("/gcloud/print_identity_token")]
async fn gcloud_print_identity_token() -> web::Json<GetGcloudPrintIdentityTokenResponce> {
    let token = if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .arg("/C")
            .arg("gcloud")
            .arg("auth")
            .arg("print-identity-token")
            .output()
            .expect("Error: 'gcloud auth print-identity-token'");
        let mut token = String::from_utf8_lossy(&output.stdout).to_string();
        token.pop();
        token.pop();
        token
    } else {
        let output = Command::new("gcloud")
            .arg("auth")
            .arg("print-identity-token")
            .output()
            .expect("Error: 'gcloud auth print-identity-token'");
        let mut token = String::from_utf8_lossy(&output.stdout).to_string();
        token.pop();
        token
    };

    let resp: GetGcloudPrintIdentityTokenResponce = GetGcloudPrintIdentityTokenResponce { token };

    actix_web::web::Json(resp)
}

#[ctor::ctor]
fn init() {
    // dotenv().ok();
    logging::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("app start");
    println!("http://localhost:9090/gcloud/print_identity_token");
    HttpServer::new(|| {
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
            .wrap(middleware::cors::cors())
            .service(gcloud_print_identity_token)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
