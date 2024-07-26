//! Contains data models of [Molecule], ...
//!
//! All those data model struct support [diesel] and [serde], such that an import/export from
//! CSV into/out of the postgres database is possible.

use std::fmt::Display;

use diesel::prelude::*;
/// A molecule description from the thermodynamic property database.
#[derive(Clone, Debug, Default, Queryable, Insertable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::molecules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(molecule_id))]
pub struct Molecule {
    #[diesel(skip_insertion)]
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
