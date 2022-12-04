use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
pub mod admin_data;
pub mod ticket_data;
pub mod radar_data;
pub mod vehicle_data;
pub mod driver_data;
pub mod args;

//boiler plate :D
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("fix your .env idot");
    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
