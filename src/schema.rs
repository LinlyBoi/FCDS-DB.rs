// @generated automatically by Diesel CLI.

diesel::table! {
    admin_emails (admin, email) {
        admin -> Int4,
        email -> Text,
    }
}

diesel::table! {
    admins (ssn) {
        ssn -> Int4,
        name -> Text,
        address -> Text,
    }
}

diesel::table! {
    auto_issued_tickets (ticket) {
        ticket -> Int4,
        vehicle -> Text,
        driver -> Nullable<Int4>,
        radar -> Int4,
    }
}

diesel::table! {
    driver_phones (driver, number) {
        driver -> Int4,
        number -> Int4,
    }
}

diesel::table! {
    drivers (ssn) {
        ssn -> Int4,
        name -> Text,
        address -> Text,
        reg_date -> Date,
        birthdate -> Date,
    }
}

diesel::table! {
    issued_tickets (ticket) {
        ticket -> Int4,
        vehicle -> Text,
        driver -> Nullable<Int4>,
        officer -> Int4,
    }
}

diesel::table! {
    officer_emails (officer, email) {
        officer -> Int4,
        email -> Text,
    }
}

diesel::table! {
    officer_phones (officer, number) {
        officer -> Int4,
        number -> Int4,
    }
}

diesel::table! {
    officers (badge_num) {
        badge_num -> Int4,
        ssn -> Text,
        jurisdiction -> Nullable<Text>,
    }
}

diesel::table! {
    radars (serialnumber) {
        serialnumber -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        category -> Text,
        description -> Text,
        issue_date -> Date,
    }
}

diesel::table! {
    vehicles (plate_num) {
        model -> Nullable<Text>,
        color -> Nullable<Text>,
        chasse_num -> Nullable<Int4>,
        plate_num -> Text,
        vehicle_type -> Text,
        category -> Text,
        owner -> Nullable<Int4>,
    }
}

diesel::joinable!(admin_emails -> admins (admin));
diesel::joinable!(auto_issued_tickets -> drivers (driver));
diesel::joinable!(auto_issued_tickets -> radars (radar));
diesel::joinable!(auto_issued_tickets -> tickets (ticket));
diesel::joinable!(auto_issued_tickets -> vehicles (vehicle));
diesel::joinable!(driver_phones -> drivers (driver));
diesel::joinable!(issued_tickets -> drivers (driver));
diesel::joinable!(issued_tickets -> officers (officer));
diesel::joinable!(issued_tickets -> tickets (ticket));
diesel::joinable!(issued_tickets -> vehicles (vehicle));
diesel::joinable!(officer_emails -> officers (officer));
diesel::joinable!(officer_phones -> officers (officer));
diesel::joinable!(vehicles -> drivers (owner));

diesel::allow_tables_to_appear_in_same_query!(
    admin_emails,
    admins,
    auto_issued_tickets,
    driver_phones,
    drivers,
    issued_tickets,
    officer_emails,
    officer_phones,
    officers,
    radars,
    tickets,
    vehicles,
);
