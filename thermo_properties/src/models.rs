//! Contains data models of [Molecule], ...
//!
//! All those data model struct support [diesel] and [serde], such that an import/export from
//! CSV into/out of the postgres database is possible.

use std::fmt::Display;

use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::antoine_coeff as antoine_db;
use crate::schema::molecules as mol_db;
use antoine_db::dsl as antoine_dsl;
use mol_db::dsl as mol_dsl;

/// A molecule description from the thermodynamic property database.
#[derive(
    Clone, Debug, Default, Queryable, Insertable, Selectable, Identifiable, AsChangeset, Deserialize,
)]
#[diesel(table_name = crate::schema::molecules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(molecule_id))]
pub struct Molecule {
    #[diesel(skip_insertion)]
    #[serde(skip)]
    /// todo
    pub molecule_id: i32,

    /// the user-friendly name of the molecule
    pub name: String,

    /// the chemical formula of the molecule
    pub formula: String,

    /// the density of the molecule at normal condtions (25° celsius and 1 atm)
    pub density: Option<f64>,

    /// the molar mass of the molecule at normal condtions (25° celsius and 1 atm)
    pub molar_mass: Option<f64>,

    /// the acentric factor of the molecule describing how near it's shape is to a sphere at normal condtions (25° celsius and 1 atm)
    pub acentric_factor: Option<f64>,

    /// the melting point of the molecule at normal conditions (25° celsius and 1 atm)
    pub melting_point: Option<f64>,

    /// the boiling point of the molecule  at normal conditions (25° celsius and 1 atm)
    pub boiling_point: Option<f64>,

    /// the criticial temperature of the molecule
    pub critical_temperature: Option<f64>,

    /// the critical pressure of the molecule
    pub critical_pressure: Option<f64>,
}

impl Molecule {
    pub fn new(name: String, formula: String) -> Self {
        Molecule {
            name,
            formula,
            ..Default::default()
        }
    }

    pub fn new_with_mut<F>(name: String, formula: String, mutator: F) -> Self
    where
        F: Fn(&mut Molecule),
    {
        let mut reval = Molecule {
            name,
            formula,
            ..Default::default()
        };
        mutator(&mut reval);
        reval
    }
}

macro_rules! option_to_str_or_na {
    ( $o:expr ) => {
        $o.map(|v| v.to_string()).unwrap_or("NA".to_owned())
    };
}

impl Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}):\n", self.name, self.formula)?;
        write!(
            f,
            "Smelting/Boiling Point: ({:?}/{}) K°\n",
            option_to_str_or_na!(self.melting_point),
            option_to_str_or_na!(self.boiling_point)
        )?;
        write!(
            f,
            "Critical T/P:({:?}/{:?})\n",
            option_to_str_or_na!(self.critical_temperature),
            option_to_str_or_na!(self.critical_temperature)
        )?;
        write!(
            f,
            "Moleculare Weight/Density: ({:?}/{:?})\n",
            option_to_str_or_na!(self.molar_mass),
            option_to_str_or_na!(self.density)
        )
    }
}

#[derive(Clone, Debug, Default, Queryable, Insertable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::antoine_coeff)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AntoineCoeff {
    #[diesel(skip_insertion)]
    pub id: i32,

    pub molecule_id: i32,

    pub min_temp: f64,

    pub max_temp: f64,

    pub a: f64,

    pub b: f64,

    pub c: f64,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AntoineCoeffCSV {
    pub name: String,

    pub min_temp: f64,

    pub max_temp: f64,

    pub a: f64,

    pub b: f64,

    pub c: f64,
}

pub struct ConnectionAware<'a, T> {
    pub data: T,
    pub conn: &'a mut PgConnection,
}

impl<'a, T> ConnectionAware<'a, T> {
    fn from_with_conn(data: T, conn: &'a mut PgConnection) -> Self {
        ConnectionAware { data, conn }
    }
}

impl<'a, T> ConnectionAware<'a, T> {
    pub fn new(data: T, conn: &'a mut PgConnection) -> Self {
        Self {
            data: data,
            conn: conn,
        }
    }
}

impl<'a> TryFrom<ConnectionAware<'a, AntoineCoeffCSV>> for AntoineCoeff {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: ConnectionAware<'a, AntoineCoeffCSV>) -> Result<Self, Self::Error> {
        let records: Vec<Molecule> = mol_dsl::molecules
            .filter(mol_db::name.eq(value.data.name.as_str()))
            .select(crate::models::Molecule::as_select())
            .load(value.conn)?;

        if records.len() != 1 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "molecules table has not exactly one record for 'name={}' but {}",
                    value.data.name.as_str(),
                    records.len()
                )
                .as_str(),
            )));
        }

        // we know we have one molecule
        let record = records.into_iter().next().unwrap();

        Ok(AntoineCoeff {
            id: 0,
            molecule_id: record.molecule_id,
            min_temp: value.data.min_temp,
            max_temp: value.data.max_temp,
            a: value.data.a,
            b: value.data.b,
            c: value.data.c,
        })
    }
}
