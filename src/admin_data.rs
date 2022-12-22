use crate::models::{Admin, AdminEmail, NewAdmin, NewAdminEmail};
use diesel::prelude::*;

pub fn listadmins(connection: &mut PgConnection) -> String {
    use crate::schema::admins::dsl::*;
    let query = admins.load::<Admin>(connection).expect("KANKER");
    let admin_list = query
        .iter()
        .map(|admin| format!("{} {}\n", admin.name, admin.address))
        .collect();
    return admin_list;
}
pub fn addmin(connection: &mut PgConnection, new_admin: NewAdmin) {
    use crate::schema::admins::dsl::*;
    diesel::insert_into(admins)
        .values(&new_admin)
        .execute(connection)
        .expect("LOL DEAD NOT WORKING INSERTION");
}
pub fn addmail(connection: &mut PgConnection, admin_email: String, inserted_id: i32) {
    use crate::schema::admin_emails::dsl::*;
    let inserted_email = NewAdminEmail {
        admin_id: &inserted_id,
        email: &admin_email,
    };
    diesel::insert_into(admin_emails)
        .values(inserted_email)
        .execute(connection)
        .expect("You FOOL! You didn't put an email in there");
}

pub fn listadminmails(connection: &mut PgConnection) {
    use crate::schema::admin_emails::dsl::*;
    let results = admin_emails
        .load::<AdminEmail>(connection)
        .expect("hecc D:");
    for addmail in results {
        println!("{} {}", addmail.email, addmail.admin_id);
    }
}
