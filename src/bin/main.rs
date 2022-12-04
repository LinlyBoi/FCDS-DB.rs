
use database_connection::admin_data::listadmins;
use database_connection::driver_data::listdrivers;
use database_connection::establish_connection;
use database_connection::args::FCDBArgs;
use clap::Parser;
fn main() {
    let connection = &mut establish_connection();
    let args: FCDBArgs = FCDBArgs::parse();
    match args.command.as_str() {
        "admins" => listadmins(connection),
        "drivers" => listdrivers(connection),
        _ => println!("You think this is funny?"),
    }


    }

