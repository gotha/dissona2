use actix_web::web;

mod health;
mod projects;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(health::health_check))
            .route("/metrics", web::get().to(health::metrics))
            .service(
                web::scope("/api")
                    .route("/projects", web::get().to(projects::list_projects))
                    .route("/projects", web::post().to(projects::create_project))
                    .route("/projects/{id}", web::get().to(projects::get_project))
                    .route("/projects/{id}", web::put().to(projects::update_project))
                    .route("/projects/{id}", web::delete().to(projects::delete_project))
                    .route(
                        "/projects/{id}/documents",
                        web::post().to(projects::add_document),
                    )
                    .route(
                        "/projects/{id}/generate/audiobook",
                        web::post().to(projects::generate_audiobook),
                    )
                    .route(
                        "/projects/{id}/generate/podcast",
                        web::post().to(projects::generate_podcast),
                    ),
            ),
    );
}
