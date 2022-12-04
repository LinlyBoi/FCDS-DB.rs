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
struct NewAdmin<'a> {
    pub id: i32,
    pub name: &'a str,
    pub address: &'a str
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
pub struct AutoIssuedTicket {
    ticket: i32,
    vehicle: String,
    driver: Option<i32>,
    radar: i32,
}

//Drivers
#[derive(Queryable,AsChangeset,Identifiable)]
pub struct Driver {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub reg_date: PgDate,
    pub birthdate: PgDate,
}

#[derive(Insertable)]
#[diesel(table_name = drivers)]
pub 
struct NewDriver<'a> {
    pub id: i32,
    pub name: &'a str,
    pub address: &'a str,
    reg_date: PgDate,
    birthdate: PgDate,
}

//Vehicles
#[derive(Queryable,AsChangeset)]
#[diesel(belongs_to(Driver))]
pub struct Vehicle {
    pub model: Option<String>,
    pub color: Option<String>,
    pub chasse_num: i32,
    pub plate_num: String,
    pub vehicle_type: String,
    pub owner: i32

}
