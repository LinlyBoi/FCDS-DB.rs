use diesel::prelude::*;
use crate::models::{Admin,NewAdmin,NewAdminEmail, AdminEmail};

pub fn listadmins(connection: &mut PgConnection)
{
    use crate::schema::admins::dsl::*;
    let query = admins.load::<Admin>(connection).expect("KANKER");
    for admin in query 
    {
        println!("{} {}",admin.name ,admin.address);
    }
}
pub fn addmin(connection: &mut PgConnection, new_admin: NewAdmin)
{
    use crate::schema::admins::dsl::*;
    diesel::insert_into(admins)
        .values(&new_admin)
        .execute(connection)
        .expect("LOL DEAD NOT WORKING INSERTION");

}
pub fn addmail(connection: &mut PgConnection, admin_email: NewAdminEmail)
{
    use crate::schema::admin_emails::dsl::*;
    diesel::insert_into(admin_emails)
    .values(&admin_email)
    .execute(connection)
    .expect("You FOOL! You didn't put an email in there");
}

pub fn listadminmails(connection: &mut PgConnection)
{
    use crate::schema::admin_emails::dsl::*;
    let results = admin_emails.load::<AdminEmail>(connection).expect("hecc D:");
    for addmail in results
    {
            println!("{} {}", addmail.email , addmail.admin_id);
    }
}
