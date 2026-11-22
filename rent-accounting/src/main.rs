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
    let connect_result = Client::connect(&connection_string.to_owned(), NoTls);

    if !connect_result.is_ok() {
        println!("database connection failed");
        return;
    }

    let mut database_client = connect_result.unwrap();

    let query_result = database_client.query("SELECT name FROM tenant", &[]);

    if !query_result.is_ok() {
        println!("query failed");
        return;
    }

    for row in query_result.unwrap() {
        let name = row.get::<&str, &str>("name");
    
        println!("tenant: {name}");
    }
}
