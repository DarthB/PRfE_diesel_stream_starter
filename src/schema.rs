// @generated automatically by Diesel CLI.

diesel::table! {
    molecules (molecule_id) {
        molecule_id -> Int4,
        name -> Nullable<Text>,
        formula -> Nullable<Text>,
        density -> Nullable<Float8>,
        molar_mass -> Nullable<Float8>,
        acentric_factor -> Nullable<Float8>,
        melting_point -> Nullable<Float8>,
        boiling_point -> Nullable<Float8>,
        critical_temperature -> Nullable<Float8>,
        critical_pressure -> Nullable<Float8>,
    }
}
