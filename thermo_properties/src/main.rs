use std::{
    error::Error,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use diesel::prelude::*;
use models::Molecule;

mod models;
mod schema;

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
                    Commands::DeleteAll => println!("Delete not implemented"),
                    Commands::Import { csv_path } => println!("Delete not implemented"),
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
