use actix_web::web;

mod google;
mod health;
mod refresh;
mod verify;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(health::health_check))
            .service(
                web::scope("/auth")
                    .route("/google", web::get().to(google::google_login))
                    .route("/callback/google", web::get().to(google::google_callback))
                    .route("/refresh", web::post().to(refresh::refresh_token))
                    .route("/verify", web::get().to(verify::verify_token))
                    .route("/logout", web::post().to(refresh::logout)),
            ),
    );
}
