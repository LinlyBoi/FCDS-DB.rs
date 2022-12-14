use crate::models::{AutoIssuedTicket, IssuedTicket, NewTicket, Ticket};
use common::CommonTicket;
use diesel::prelude::*;

pub fn create_ticket(connection: &mut PgConnection, ticket: NewTicket) {
    use crate::schema::tickets::dsl::*;
    diesel::insert_into(tickets)
        .values(&ticket)
        .execute(connection)
        .expect("Didn't save ticket AAAAAA");
}

pub fn get_tickets(connection: &mut PgConnection, amount: i64) -> Vec<CommonTicket> {
    use crate::schema::tickets::dsl::*;
    let results = tickets
        .limit(amount)
        .load::<Ticket>(connection)
        .expect("KANKER TIKET");
    return results
        .iter()
        .map(|ticket| CommonTicket {
            id: ticket.id,
            category: String::from(&ticket.category),
            description: String::from(&ticket.description),
        })
        .collect(); //Shoves everything to a vector
}

pub fn get_ticket(connection: &mut PgConnection, tickid: i32) -> CommonTicket {
    use crate::schema::tickets::dsl::*;
    let ticket = &mut tickets
        .filter(id.eq(tickid))
        .limit(1)
        .load::<Ticket>(connection)
        .expect("no tickets  :(")[0];
    return CommonTicket {
        id: ticket.id,
        category: String::from(&ticket.category),
        description: String::from(&ticket.description),
    };
}

pub fn issue_ticket(connection: &mut PgConnection, issued_ticket: IssuedTicket) {
    use crate::schema::issued_tickets::dsl::*;
    diesel::insert_into(issued_tickets)
        .values(issued_ticket)
        .execute(connection)
        .expect("PAINNNN IT NO WORKEY (issued tickets)");
}

pub fn auto_issue_ticket(connection: &mut PgConnection, issued_ticket: AutoIssuedTicket) {
    use crate::schema::auto_issued_tickets::dsl::*;
    diesel::insert_into(auto_issued_tickets)
        .values(issued_ticket)
        .execute(connection)
        .expect("PAINNNN IT NO WORKEY (issued tickets)");
}
