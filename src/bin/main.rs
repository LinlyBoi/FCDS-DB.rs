use actix_web::{
    get,
    web::{self, Json},
    App, HttpServer,
};
use backend::{establish_connection, ticket_data::get_ticket};
use common::CommonTicket;


// //Admin Services
// #[get("/api/admin/{id}")]
// async fn admin(id: web::Path<i32>) -> Json<CommonAdmin> {
//     Json(get_admin(&mut establish_connection(), *id))
// }
// #[get("/api/admins{amount}")]
// async fn admins(amount: web::Path<i64>) -> Json<Vec<CommonAdmin>> {
//     Json(get_admins(&mut establish_connection(), *amount))
// }


//Ticket Table Services
#[get("api/ticket/{id}")]
async fn ticket(id: web::Path<i32>) -> Json<CommonTicket> {
    let fetched_ticket = get_ticket(&mut establish_connection(), *id);
    Json(fetched_ticket)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ticket))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
