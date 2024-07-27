use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::molecules;

#[derive(Insertable, Debug, Clone, Default, Deserialize)]
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
