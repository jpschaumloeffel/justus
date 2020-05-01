extern crate diesel;
extern crate justus;

use self::diesel::prelude::*;
use self::justus::*;
use self::models::Account;
use std::env::args;

use diesel::data_types::Cents;

fn main() {
    use justus::schema::accounts::dsl::{accounts, balance};

    let id = args()
        .nth(1)
        .expect("update_account requires an account id")
        .parse::<i32>()
        .expect("Invalid id");

    let increment = args()
        .nth(2)
        .expect("update_account requires an increment")
        .parse::<f32>()
        .expect("invalid increment");

    let connection = establish_connection();

    // find account
    let acct = accounts
        .find(id)
        .get_result::<Account>(&connection)
        .expect(&format!("Unable to find account {}", id));

    // calculate new balance
    let new_balance = acct.balance + Cents((increment * 100.0) as i64);

    // update account
    let account = diesel::update(accounts.find(id))
        .set(balance.eq(new_balance))
        .get_result::<Account>(&connection)
        .expect(&format!("Unable to find account {}", id));

    println!(
        "updated account {}: {} -> {}",
        account.id, account.name, account.balance.0
    );
}
