use database_connection::establish_connection;

use database_connection::admin_data::listadmins;
// use database_connection::models::NewAdmin;
fn main() {
    let connection = &mut establish_connection();
    listadmins(connection);
    // let ggsya = NewAdmin {
    //     ssn: 69421,
    //     name: String::from("GGsya"),
    //     address: String::from("his mom's basement"),
    // };

    // listadmins(connection);
    }

