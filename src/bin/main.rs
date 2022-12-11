
use backend::admin_data::listadmins;
use backend::establish_connection;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use backend::ticket_data::get_latest_ticket;

#[get("/ggsya")]
async fn ggsya() -> impl Responder {
    HttpResponse::Ok().body("Ggsya is here")
}
#[get("/admins")]
async fn admins() -> impl Responder {

    HttpResponse::Ok().body(listadmins(&mut establish_connection()))
}
#[get("/ticket")]
async fn ticket() -> impl Responder {
    HttpResponse::Ok().body(get_latest_ticket(&mut establish_connection()))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ggsya)
            .service(admins)
            .service(ticket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
