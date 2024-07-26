#![recursion_limit = "512"]
use std::{
    error::Error,
    fs::File,
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueEnum};
use diesel::dsl::insert_into;
use diesel::prelude::*;

use models::{AntoineCoeff, ConnectionAware, Molecule};
use schema::molecules::{formula, molecule_id};

mod models;
mod schema;

use crate::schema::antoine_coeff::dsl as antoine_dsl;
use crate::schema::molecules as mol_db;
use crate::schema::molecules::dsl as mol_dsl;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// creates a new molecule
    Create { name: String, comp_formula: String },

    /// reads a specific or all molecules
    Read {
        /// reads a molecule by ID, is prefered over formula
        #[arg(short, conflicts_with = "search_formula")]
        id: Option<i32>,

        /// reads a molecule by it's formula
        #[arg(short, conflicts_with = "id")]
        search_formula: Option<String>,
    },

    /// not implemented yet
    Update,

    /// not implemented yet
    Delete,

    /// deletes all rows in every table
    DeleteAll,

    /// Imports information from a CSV file
    Import {
        csv_path: PathBuf,

        #[arg(short)]
        format: Option<CSVFormats>,
    },

    /// Exits the interactive shell
    Quit,
}

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum CSVFormats {
    #[default]
    Molecules,
    Antoine,
    NrtlBinary,
}

pub fn create_molecule<F>(
    conn: &mut PgConnection,
    symbol_formula: String,
    name: String,
    mutator: F,
) -> Molecule
where
    F: Fn(&mut Molecule),
{
    let mut new_mol = models::Molecule::new(name, symbol_formula);
    mutator(&mut new_mol);

    diesel::insert_into(schema::molecules::table)
        .values(&new_mol)
        .returning(Molecule::as_returning())
        .get_result(conn)
        .expect("Error saving new Molecule")
}

fn cmd_create(conn: &mut PgConnection, name: String, symbol_formula: String) {
    let new_mol = models::Molecule::new_with_mut(name, symbol_formula, |m| {
        // just an example how we can adapt the data
        m.boiling_point = Some(100.0);
    });

    diesel::insert_into(schema::molecules::table)
        .values(&new_mol)
        .returning(Molecule::as_returning())
        .get_result(conn)
        .expect("Error saving new Molecule");
}

pub fn read_molecules_from_csv<T: AsRef<Path>>(
    file_path: T,
    print_flag: bool,
) -> core::result::Result<Vec<models::Molecule>, Box<dyn Error>> {
    let file = File::open(file_path.as_ref())?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut reval = vec![];
    for result in rdr.deserialize() {
        let record: models::Molecule = result?;
        if print_flag {
            println!("{:?}", record);
        }
        reval.push(record);
    }
    Ok(reval)
}

pub fn read_antoine_from_csv<T: AsRef<Path>>(
    file_path: T,
    print_flag: bool,
) -> core::result::Result<Vec<models::AntoineCoeffCSV>, Box<dyn Error>> {
    let file = File::open(file_path.as_ref())?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut reval = vec![];
    for result in rdr.deserialize() {
        let record: models::AntoineCoeffCSV = result?;
        if print_flag {
            println!("{:?}", record);
        }
        reval.push(record);
    }
    Ok(reval)
}

/*
/// error[E0275]: overflow evaluating the requirement
fn tbl_import<T, R>(conn: &mut PgConnection, table: T, records: &Vec<R>)
where
    T: Table,
    R: Insertable<T> + Sized,
{
    let res = insert_into(table).values(records).execute(conn);
    match res {
        Ok(num) => println!("Inserted {} rows of molecules.", num),
        Err(e) => println!("Import to Postgres failed: {}", e.to_string()),
    }
}
*/

fn cmd_import(
    conn: &mut PgConnection,
    csv_path: PathBuf,
    format: CSVFormats,
) -> Result<(), Box<dyn Error>> {
    match format {
        CSVFormats::Molecules => {
            let records = read_molecules_from_csv(csv_path.as_path(), true)?;
            insert_into(mol_dsl::molecules)
                .values(records)
                .execute(conn)?;
        }
        CSVFormats::Antoine => {
            let records = read_antoine_from_csv(csv_path.as_path(), true)?;
            println!("{:?}", records);
            let results: Vec<Result<AntoineCoeff, Box<dyn Error>>> = records
                .into_iter()
                .map(|e| ConnectionAware::new(e, conn).try_into())
                .collect();

            if results.iter().any(|e| e.is_err()) {
                // todo extract errors form iterator
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("not all coefficeints from CSV could be mapped to molecules",).as_str(),
                )));
            }

            let records: Vec<AntoineCoeff> = results.into_iter().map(|e| e.unwrap()).collect();

            insert_into(antoine_dsl::antoine_coeff)
                .values(records)
                .execute(conn)?;
        }
        CSVFormats::NrtlBinary => println!("Nrtl Binary import not supported yet."),
    }

    Ok(())
}

fn cmd_read(conn: &mut PgConnection, _id: Option<i32>, _formula: Option<String>) {
    if _id.is_some() && _formula.is_some() {
        panic!("That should never happen.");
    }

    let results: Vec<Molecule> = if _id.is_some() {
        mol_dsl::molecules
            .filter(molecule_id.eq(_id.unwrap()))
            .select(crate::models::Molecule::as_select())
            .load(conn)
            .expect("ERR")
    } else if _formula.is_some() {
        mol_dsl::molecules
            .filter(formula.eq(_formula.unwrap()))
            .select(crate::models::Molecule::as_select())
            .load(conn)
            .expect("ERR")
    } else {
        mol_dsl::molecules
            .select(crate::models::Molecule::as_select())
            .load(conn)
            .expect("Error loading molecules.")
    };

    println!("Displaying {}. Molecules:", results.len());
    for molecule in results {
        println!("{}", molecule);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Running in {:?}",
        std::env::current_dir().ok().unwrap().to_str()
    );
    dotenvy::from_filename("./.env").expect("found file");

    let db_url = dotenvy::var("DATABASE_URL").expect("Postgres DATABASE URL not given");
    let mut conn = PgConnection::establish(&db_url)?;
    println!("Connection established - starting interactive CLI");

    // get CLI given at program start
    let mut cli = Some(Cli::parse());
    // interactive cli loop
    loop {
        if let Some(cli) = cli {
            if let Some(cmd) = cli.command {
                match cmd {
                    Commands::Create { name, comp_formula } => {
                        cmd_create(&mut conn, name, comp_formula)
                    }
                    Commands::Read { id, search_formula } => {
                        cmd_read(&mut conn, id, search_formula)
                    }
                    Commands::Update => println!("Update not implemented"),
                    Commands::Delete => println!("Delete not implemented"),
                    Commands::DeleteAll => {
                        diesel::delete(mol_db::table).execute(&mut conn)?;
                    }
                    Commands::Import { csv_path, format } => {
                        cmd_import(&mut conn, csv_path, format.unwrap_or(CSVFormats::Molecules))?;
                    }

                    Commands::Quit => break,
                }
            } else {
                println!("No Command given");
            }
        }

        let mut line = String::new();
        print!("<CLI> ");
        stdout().flush()?;
        stdin()
            .read_line(&mut line)
            .expect("readline does not work");

        line = "cli ".to_owned() + &line;
        let iterable = line.trim().split(' ');
        cli = match Cli::try_parse_from(iterable.into_iter()) {
            Ok(cli) => Some(cli),
            Err(e) => {
                println!("{}", e);
                None
            }
        };
    }

    Ok(())
}
