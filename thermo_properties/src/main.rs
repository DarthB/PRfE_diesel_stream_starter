use std::error::Error;

use diesel::{Connection, PgConnection};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Running in {:?}",
        std::env::current_dir().ok().unwrap().to_str()
    );
    dotenvy::from_filename("./.env").expect("found file");
    println!("Hello, world!");

    /*
    for (key, val) in std::env::vars() {
        println!("{} = {}", key, val);
    }
    */

    let db_url = dotenvy::var("DATABASE_URL").expect("Postgres DATABASE URL not given");

    PgConnection::establish(&db_url)?;

    println!("Connection established!");
    Ok(())
}
