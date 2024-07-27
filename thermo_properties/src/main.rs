use std::error::Error;

use diesel::{Connection, PgConnection};

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::from_filename("./.env")?;

    let db_url = dotenvy::var("DATABASE_URL")?;
    let res = PgConnection::establish(&db_url);
    match res {
        Ok(_) => println!("Connection Established"),
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}
