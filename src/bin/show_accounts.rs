extern crate diesel;
extern crate justus;

use self::diesel::prelude::*;
use self::models::*;
use justus::*;

// use diesel::data_types::Cents;

fn main() {
    use justus::schema::accounts::dsl::*;

    let connection = establish_connection();
    let results = accounts
        .limit(5)
        .load::<Account>(&connection)
        .expect("Error loading accounts");

    println!("displaying {} accounts", results.len());
    for account in results {
        /*
        let balance = match account.balance {
            Cents{ x } => x,
        };
        */
        println!("{} - {:.2}", account.name, account.balance.0 as f64 / 100.0);
    }
}
