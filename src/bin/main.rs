use database_connection::establish_connection;

use database_connection::admin_data::listadmins;
fn main() {
    let connection = &mut establish_connection();
    listadmins(connection);
}
