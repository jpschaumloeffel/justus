extern crate justus;
extern crate diesel;

use self::justus::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Enter Account Name:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)];

    println!("\nOk...");

    let account = create_account(&connection, name);
    println!("\nSaved account with id {}", account.id);
}

