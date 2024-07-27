use std::{error::Error, fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    Connection, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};
use models::Molecule;

mod models;
mod schema;

use mol_sch::dsl as mol_dsl;
use schema::molecules as mol_sch;

#[derive(Debug, Parser)]
struct CliArgs {
    #[command(subcommand)]
    sub_command: SubCommands,
}

#[derive(Debug, Clone, Subcommand)]
enum SubCommands {
    Create,
    Import,
    Read {
        #[arg(short, long, conflicts_with = "formula")]
        id: Option<i32>,

        #[arg(short, long, conflicts_with = "id")]
        formula: Option<String>,
    },
    Update,
    DeleteAll,
}

fn cmd_read(
    id: Option<i32>,
    formula: Option<String>,
    conn: &mut PgConnection,
) -> Result<(), Box<dyn Error>> {
    if id.is_some() && formula.is_some() {
        panic!("called with both")
    }

    let records = if let Some(id) = id {
        mol_dsl::molecules
            .filter(mol_sch::molecule_id.eq(id))
            .select(Molecule::as_select())
            .load(conn)?
    } else if let Some(formula) = formula {
        mol_dsl::molecules
            .filter(mol_sch::formula.eq(formula))
            .select(Molecule::as_select())
            .load(conn)?
    } else {
        // schema::molecules::dsl::*
        let records: Vec<Molecule> = mol_dsl::molecules
            .select(Molecule::as_select())
            .load(conn)?;
        records
    };

    // SELECT * FROM molecues

    for record in records {
        println!("{:?}", record);
    }

    Ok(())
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
        SubCommands::Read { id, formula } => {
            cmd_read(id, formula, &mut conn);
        }
        SubCommands::Update => todo!(),
        SubCommands::DeleteAll => {
            diesel::delete(mol_dsl::molecules).execute(&mut conn)?;
            println!("deleted all");
        }
    }

    Ok(())
}
