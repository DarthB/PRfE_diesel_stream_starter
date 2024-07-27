use std::error::Error;

use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::{antoine_coeff, molecules};

use crate::schema::molecules as mol_sch;
use mol_sch::dsl as mol_dsl;

use crate::schema::antoine_coeff as ant_sch;
use ant_sch::dsl as ant_dsl;

#[derive(
    Insertable,
    Selectable,
    Identifiable,
    Queryable,
    QueryableByName,
    Debug,
    Clone,
    Default,
    Deserialize,
)]
#[diesel(primary_key(molecule_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name=molecules)]
pub struct Molecule {
    #[diesel(skip_insertion)]
    #[serde(skip)]
    pub molecule_id: i32,

    pub name: String,
    pub formula: String,
    pub density: Option<f64>,
    pub molar_mass: Option<f64>,
    pub acentric_factor: Option<f64>,
    pub melting_point: Option<f64>,
    pub boiling_point: Option<f64>,
    pub critical_temperature: Option<f64>,
    pub critical_pressure: Option<f64>,
}

impl Molecule {
    pub fn new_with_mutator<'a, F>(name: String, formula: String, mutator: F) -> Self
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

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AntoineCoeffCSV {
    pub name: String,
    pub low_temp: f64,
    pub max_temp: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

#[derive(Insertable, Selectable, Identifiable, Queryable, Debug, Clone, Default, Deserialize)]
//#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name=antoine_coeff)]
pub struct AntoineCoeff {
    #[diesel(skip_insertion)]
    pub id: i32,
    pub mol_id: i32,
    pub low_temp: f64,
    pub max_temp: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub struct ConnectionAware<'a, T: Sized> {
    pub data: T,

    pub conn: &'a mut PgConnection,
}

impl<'a, T> ConnectionAware<'a, T> {
    pub fn new_with_conn(data: T, conn: &'a mut PgConnection) -> Self {
        ConnectionAware { data, conn }
    }
}

// Ethanol at 78
impl<'a> TryFrom<ConnectionAware<'a, AntoineCoeffCSV>> for AntoineCoeff {
    type Error = Box<dyn Error>;

    fn try_from(value: ConnectionAware<'a, AntoineCoeffCSV>) -> Result<Self, Self::Error> {
        let res: Vec<Molecule> = mol_dsl::molecules
            .filter(mol_sch::name.eq(value.data.name))
            .select(Molecule::as_select())
            .load(value.conn)?;

        if res.len() != 1 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unambgious: we have {} records", res.len()).as_str(),
            )));
        }

        Ok(AntoineCoeff {
            id: 0,
            mol_id: res.into_iter().nth(0).unwrap().molecule_id,
            low_temp: value.data.low_temp,
            max_temp: value.data.max_temp,
            a: value.data.a,
            b: value.data.b,
            c: value.data.c,
        })
    }
}
