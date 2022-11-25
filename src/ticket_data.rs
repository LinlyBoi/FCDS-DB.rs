use diesel::prelude::*;
use crate::models::{NewTicket,Ticket,IssuedTicket, AutoIssuedTicket};

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

pub fn issue_ticket(connection: &mut PgConnection,issued_ticket: IssuedTicket)
{
    use crate::schema::issued_tickets::dsl::*;
    diesel::insert_into(issued_tickets)
    .values(issued_ticket).execute(connection).expect("PAINNNN IT NO WORKEY (issued tickets)");

}

pub fn auto_issue_ticket(connection: &mut PgConnection,issued_ticket: AutoIssuedTicket)
{
    use crate::schema::auto_issued_tickets::dsl::*;
    diesel::insert_into(auto_issued_tickets)
    .values(issued_ticket).execute(connection).expect("PAINNNN IT NO WORKEY (issued tickets)");

}