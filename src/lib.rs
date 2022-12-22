use std::env;
pub mod admin_data;
pub mod driver_data;
pub mod models;
pub mod radar_data;
pub mod schema;
pub mod ticket_data;
pub mod vehicle_data;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

//boiler plate :D
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("fix your .env idot");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
//
//Pool instead

// pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
// fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     Pool::builder().build(manager)
// }
//
// pub fn establish_connection() -> PgPool {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("pls obama put the link this time");
//     init_pool(&database_url).expect("Failed to create pool")
// }
