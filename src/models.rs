use diesel::{prelude::*, data_types::PgDate};
use crate::schema::*;



//Admins
#[derive(Queryable,Debug,AsChangeset)]
pub 
struct Admin {
    pub ssn: i32,
    pub name: String,
    pub address: String
}
#[derive(Insertable)]
#[diesel(table_name = admins)]
pub 
struct NewAdmin {
    pub ssn: i32,
    pub name: String,
    pub address: String
}
pub struct AdminEmail {
    pub admin: i32,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = admin_emails)]
struct NewAdminEmail<'a> {
    pub admin: &'a i32,
    pub email: &'a String,
}
//Ticket things
#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    pub category: String,
    pub description: String,
    pub issue_date: PgDate
}

#[derive(Insertable)]
#[diesel(table_name = tickets)]
pub struct NewTicket<'a> {
    pub category: &'a str,
    pub description: &'a str,
}

//Vehicles
#[derive(Queryable,AsChangeset)]
pub struct Vehicle {
    pub model: Option<String>,
    pub color: Option<String>,
    pub chasse_num: i32,
    pub plate_num: String,
    pub vehicle_type: String,
    pub owner: i32

}
