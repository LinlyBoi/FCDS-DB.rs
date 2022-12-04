use diesel::prelude::*;
use crate::models::{Driver,NewDriver};

pub fn listdrivers(connection: &mut PgConnection)
{
    use crate::schema::drivers::dsl::*;
    let queury = drivers.load::<Driver>(connection).expect("KANKER");
    for driver in queury 
    {
        println!("{} {}",driver.name ,driver.address);
    }
}

pub fn addriver(connection: &mut PgConnection, new_driver: NewDriver){
    
    use crate::schema::drivers::dsl::*;
    diesel::insert_into(drivers)
        .values(&new_driver)
        .execute(connection)
        .expect("Couldn't insert new driver :)");
}
