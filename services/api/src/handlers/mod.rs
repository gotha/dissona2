use actix_web::web;

mod health;
mod progress;
mod projects;
mod push;
mod samples;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(health::health_check))
            .route("/metrics", web::get().to(health::metrics))
            .service(
                web::scope("/api")
                    // Projects
                    .route("/projects", web::get().to(projects::list_projects))
                    .route("/projects", web::post().to(projects::create_project))
                    .route("/projects/upload", web::post().to(projects::upload_project))
                    .route("/projects/{id}", web::get().to(projects::get_project))
                    .route("/projects/{id}", web::put().to(projects::update_project))
                    .route("/projects/{id}", web::delete().to(projects::delete_project))
                    .route(
                        "/projects/{id}/chapters",
                        web::get().to(projects::list_chapters),
                    )
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
                    )
                    // Progress (cross-device sync)
                    .route(
                        "/projects/{id}/progress",
                        web::get().to(progress::get_progress),
                    )
                    .route(
                        "/projects/{id}/progress",
                        web::put().to(progress::update_progress),
                    )
                    // Samples
                    .route("/samples/try", web::post().to(samples::try_sample))
                    // Push notifications
                    .route("/push/subscribe", web::post().to(push::subscribe))
                    .route("/push/unsubscribe", web::delete().to(push::unsubscribe)),
            ),
    );
}
