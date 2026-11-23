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
    #[arg(long)]
    database_name: String,
}

fn main() {
    let arguments = Arguments::parse();
    let database_server = arguments.database_server;
    let database_user = arguments.database_user;
    let database_password = arguments.database_password;
    let database_name = arguments.database_name;

    println!("connecting to {database_server} with user {database_user}");

    let connection_string = format!("host={database_server} user={database_user} password={database_password} dbname={database_name}");
    let connect_result = Client::connect(&connection_string.to_owned(), NoTls);

    if !connect_result.is_ok() {
        println!("database connection failed");
        return;
    }

    let mut database_client = connect_result.unwrap();

    let transations_result = database_client.query("
    select 
        t.full_name as tenant_name,
        payment_sum.amount_incoming_sum as payment_sum,
        fee_sum.amount_incoming_sum as fee_sum,
        deposit_sum.amount_incoming_sum as deposit_sum
    from tenant t
    left join
        (
            select c.tenant as tenant_id, sum(p.amount_incoming) as amount_incoming_sum
            from contract c
            join payment p on c.id = p.contract
            group by c.tenant
        ) payment_sum
    on t.id = payment_sum.tenant_id
    left join
        (
            select c.tenant as tenant_id, sum(f.amount) as amount_incoming_sum
            from contract c
            join additional_fees f on c.id = f.contract
            group by c.tenant
        ) fee_sum
    on t.id = fee_sum.tenant_id
    left join
        (
            select c.tenant as tenant_id, sum(d.amount) as amount_incoming_sum
            from contract c
            join deposit d on c.id = d.contract
            group by c.tenant
        ) deposit_sum
    on t.id = deposit_sum.tenant_id;", &[]);

    if !transations_result.is_ok() {
        println!("query failed");
        return;
    }

    for row in transations_result.unwrap() {
        let tenant_name = row.get::<&str, &str>("tenant_name");
        let payment_sum = row.get::<&str, f64>("payment_sum");
        let fee_sum = row.get::<&str, f64>("fee_sum");
        let deposit_sum = row.get::<&str, f64>("deposit_sum");
    
        println!("{tenant_name}: payments {payment_sum}€, additional fees {fee_sum}€, deposits {deposit_sum}€");
    }
}
