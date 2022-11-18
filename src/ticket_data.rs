use diesel::prelude::*;
use crate::models::{NewTicket,Ticket};

pub fn create_ticket(connection: &mut PgConnection,ticket: NewTicket)
{
    use crate::schema::tickets::dsl::*;
    diesel::insert_into(tickets)
    .values(&ticket)
    .execute(connection)
    .expect("Didn't save ticket AAAAAA");
}

pub fn get_tickets(connection: &mut PgConnection)
{
    use crate::schema::tickets::dsl::*;
    let results = tickets.load::<Ticket>(connection).expect("KANKER TIKET");
    for ticket in results 
    {
            println!("{} {} \n", ticket.id,ticket.description);
    }
        

}

