//use postgres::{Client, NoTls};
use std::env;

fn main() {
    //let mut client = Client::connect("host=localhost user=benedikt password", NoTls)?;
    let database_password;
    let database_password_env_name = "DATABASE_PASSWORD";
    let database_password_env_value = env::var(database_password_env_name);

    if !database_password_env_value.is_ok() {
        println!("missing environment variable {}", database_password_env_name);
        return;
    }

    database_password = database_password_env_value.unwrap();

    println!("database password: {}", database_password);
}
