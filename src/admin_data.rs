use diesel::prelude::*;
use crate::models::{Admin,NewAdmin};

pub fn listadmins(connection: &mut PgConnection)
{
    use crate::schema::admins::dsl::*;
    let results = admins.load::<Admin>(connection).expect("KANKER");
    for admin in results 
    {
        println!("{} {}",admin.name ,admin.address);
    }
}
pub fn addmin(connection: &mut PgConnection, admin: NewAdmin)
{
    use crate::schema::admins::dsl::*;
    diesel::insert_into(admins)
        .values(&admin)
        .execute(connection)
        .expect("LOL DEAD NOT WORKING INSERTION");

}
