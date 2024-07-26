use std::{
    error::Error,
    fs::File,
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use diesel::dsl::insert_into;
use diesel::prelude::*;

use models::Molecule;

mod models;
mod schema;

use crate::schema::molecules;
use crate::schema::molecules::dsl as cur_dsl;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// creates a new molecule
    Create { name: String, formula: String },

    /// reads a specific or all molecules
    Read {
        /// reads a molecule by ID, is prefered over formula
        #[arg(short, conflicts_with = "formula")]
        id: Option<i32>,

        /// reads a molecule by it's formula
        #[arg(short, conflicts_with = "id")]
        formula: Option<String>,
    },

    /// not implemented yet
    Update,

    /// not implemented yet
    Delete,

    /// deletes all rows in every table
    DeleteAll,

    /// Imports information from a CSV file
    Import { csv_path: PathBuf },

    /// Exits the interactive shell
    Quit,
}

pub fn create_molecule<F>(
    conn: &mut PgConnection,
    formula: String,
    name: String,
    mutator: F,
) -> Molecule
where
    F: Fn(&mut Molecule),
{
    let mut new_mol = models::Molecule::new(name, formula);
    mutator(&mut new_mol);

    diesel::insert_into(schema::molecules::table)
        .values(&new_mol)
        .returning(Molecule::as_returning())
        .get_result(conn)
        .expect("Error saving new Molecule")
}

fn cmd_create(conn: &mut PgConnection, name: String, formula: String) {
    let new_mol = models::Molecule::new_with_mut(name, formula, |m| {
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

fn cmd_import(conn: &mut PgConnection, csv_path: PathBuf) {
    let res = read_molecules_from_csv(csv_path.as_path(), true);
    match res {
        Ok(v) => {
            // todo insert statement
            let res = insert_into(cur_dsl::molecules).values(v).execute(conn);
            match res {
                Ok(num) => println!("Inserted {} rows of molecules.", num),
                Err(e) => println!("Import to Postgres failed: {}", e.to_string()),
            }
        }
        Err(err) => {
            println!(
                "Error importing {}: {}",
                csv_path.clone().to_str().unwrap(),
                err.to_string()
            );
        }
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
                    Commands::Create { name, formula } => cmd_create(&mut conn, name, formula),
                    Commands::Read { id, formula } => println!("Delete not implemented"),
                    Commands::Update => println!("Update not implemented"),
                    Commands::Delete => println!("Delete not implemented"),
                    Commands::DeleteAll => {
                        diesel::delete(molecules::table).execute(&mut conn)?;
                    }
                    Commands::Import { csv_path } => cmd_import(&mut conn, csv_path),
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
