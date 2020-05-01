extern crate diesel;
extern crate justus;

use justus::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use justus::schema::accounts::dsl::*;

    let connection = establish_connection();
    let results = accounts.limit(5)
        .load::<Account>(&connection)
        .expect("Error loading accounts");

    println!("displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.name);
    }
}
