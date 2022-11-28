use diesel::{prelude::*, data_types::PgDate};
use crate::schema::*;



//Admins
#[derive(Queryable,Debug,AsChangeset,Identifiable)]
pub 
struct Admin {
    pub id: i32,
    pub name: String,
    pub address: String
}
#[derive(Insertable)]
#[diesel(table_name = admins)]
pub 
struct NewAdmin {
    pub id: i32,
    pub name: String,
    pub address: String
}
#[derive(Queryable,AsChangeset,Associations)]
#[diesel(belongs_to(Admin))]
pub struct AdminEmail {
    pub admin_id: i32,
    pub email: String,
}

#[derive(Insertable,Associations)]
#[diesel(belongs_to(Admin))]
#[diesel(table_name = admin_emails)]
pub struct NewAdminEmail<'a> {
    pub admin_id: &'a i32,
    pub email: &'a str,
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

#[derive(Insertable)]
#[diesel(table_name = issued_tickets)]
pub struct IssuedTicket {
    ticket: i32,
    vehicle: String,
    driver: Option<i32>,
    officer: i32,
}
#[derive(Insertable)]
#[diesel(table_name = auto_issued_tickets)]
pub struct AutoIssuedTicket<'a> {
    ticket: &'a i32,
    vehicle: &'a str,
    driver: Option<i32>,
    radar: &'a i32,
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
