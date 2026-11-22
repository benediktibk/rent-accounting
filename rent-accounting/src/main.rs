//use postgres::{Client, NoTls};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    database_server: String,
}

fn main() {
    //let mut client = Client::connect("host=localhost user=benedikt password", NoTls)?;

    let arguments = Arguments::parse();
    
    let database_server = arguments.database_server;

    println!("database server: {}", database_server);
}
