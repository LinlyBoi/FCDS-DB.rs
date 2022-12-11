
use backend::admin_data::listadmins;
use backend::driver_data::listdrivers;
use backend::establish_connection;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/ggsya")]
async fn ggsya() -> impl Responder {
    HttpResponse::Ok().body("Ggsya is here")
}
#[get("/admins")]
async fn admins() -> impl Responder {

    HttpResponse::Ok().body(listadmins(&mut establish_connection()))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ggsya)
            .service(admins)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
