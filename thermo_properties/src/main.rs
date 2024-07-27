use std::{error::Error, fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use diesel::{Connection, PgConnection, RunQueryDsl};
use models::Molecule;
use serde::Deserialize;

mod models;
mod schema;

use schema::molecules::dsl as mol_dsl;

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

fn cmd_import(path: PathBuf, conn: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    /*for record in rdr.records() {
        println!("Record:{:?}", record.unwrap());
    }*/
    let records: Vec<Molecule> = rdr
        .deserialize()
        .map(|res| res.expect("format error"))
        .collect();

    diesel::insert_into(mol_dsl::molecules)
        .values(records)
        .execute(conn)?;

    Ok(())
}

fn cmd_create(name: &str, formula: &str, conn: &mut PgConnection) {
    println!("name={}, symbol={}", name, formula);
    let record = Molecule::new_with_mutator(name.to_owned(), formula.to_owned(), |mol| {
        mol.boiling_point = Some(42.)
    });

    diesel::insert_into(mol_dsl::molecules)
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
        SubCommands::Import => {
            cmd_import("resources/molecules.csv".into(), &mut conn)?;
        }
        SubCommands::Read => todo!(),
        SubCommands::Update => todo!(),
        SubCommands::DeleteAll => {
            diesel::delete(mol_dsl::molecules).execute(&mut conn)?;
            println!("deleted all");
        }
    }

    Ok(())
}
