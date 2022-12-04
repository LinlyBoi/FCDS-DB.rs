use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug,Parser)]
pub struct FCDBArgs
    {
        /// List Admins dummy
        // pub table_type: TableSubcommand,
        pub command: String,

    }

// #[derive(Debug,Subcommand)]
// pub enum TableSubcommand {
//     Admin(AdminCommand),
//     Driver(DriverCommand),
//     Vehicle(VehicleCommand),
//     Show,
// }

#[derive(Debug,Args )]
pub struct AdminCommand {
    pub command: String
}
#[derive(Debug,Args )]
pub struct DriverCommand {
    pub command: String
}
#[derive(Debug,Args )]
pub struct VehicleCommand {
    pub command: String
}

#[derive(Debug,Args )]
pub struct TableActions {
    pub command: String
}
