use postgres::{Client, NoTls};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(long)]
    database_server: String,
    #[arg(long)]
    database_user: String,
    #[arg(long)]
    database_password: String,
}

fn main() {
    let arguments = Arguments::parse();
    let database_server = arguments.database_server;
    let database_user = arguments.database_user;
    let database_password = arguments.database_password;

    println!("connecting to {database_server} with user {database_user}");

    let connection_string = format!("host={database_server} user={database_user} password={database_password}");
    let mut database_client = Client::connect(&connection_string.to_owned(), NoTls);
}
