use env_logger::Env;
use ntex::web;

mod configs;
mod errors;
mod models;
mod routes;
mod schema;

#[ntex::main]
#[rustfmt::skip]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();
    let db_info=configs::db::load_db();
    let info=configs::server::load_server();
    log::info!("{}",info);
    log::info!("{}",db_info);
    let poll=db_info.poll();
    web::server(move || {
        web::App::new()
        .state(poll.clone())
        .wrap(web::middleware::Logger::default())
        .service(routes::index)
        .service(routes::todo::index)
        .service(routes::todo::post)
        .service(routes::todo::put)
        .service(routes::todo::del)
        .default_service(web::route().to(routes::default))
    })
    .workers(8)
    .bind(info.to_socket_addr())?
    .run()
    .await?;
    Ok(())
}
