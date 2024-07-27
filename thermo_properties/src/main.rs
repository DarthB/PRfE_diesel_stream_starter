use std::error::Error;

use clap::{Parser, Subcommand};
use diesel::{Connection, PgConnection, RunQueryDsl};
use models::Molecule;

mod models;
mod schema;

#[derive(Debug, Parser)]
struct CliArgs {
    #[command(subcommand)]
    sub_command: SubCommands,
}

#[derive(Debug, Clone, Subcommand)]
enum SubCommands {
    Create,
    Import,
    Read,
    Update,
    DeleteAll,
}

fn cmd_create(name: &str, formula: &str, conn: &mut PgConnection) {
    println!("name={}, symbol={}", name, formula);
    let record = Molecule::new_with_mutator(name.to_owned(), formula.to_owned(), |mol| {
        mol.boiling_point = Some(42.)
    });

    diesel::insert_into(schema::molecules::dsl::molecules)
        .values(record)
        .execute(conn)
        .unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::from_filename("./.env")?;

    let db_url = dotenvy::var("DATABASE_URL")?;
    let res = PgConnection::establish(&db_url);
    let mut conn = match res {
        Ok(conn) => {
            println!("Connection Established");
            conn
        }
        Err(e) => return Err(Box::new(e)),
    };

    let args = CliArgs::parse();

    match args.sub_command {
        SubCommands::Create => cmd_create("my-mol", "ACDC", &mut conn),
        SubCommands::Import => todo!(),
        SubCommands::Read => todo!(),
        SubCommands::Update => todo!(),
        SubCommands::DeleteAll => {
            diesel::delete(schema::molecules::dsl::molecules).execute(&mut conn)?;
            println!("deleted all");
        }
    }

    Ok(())
}
