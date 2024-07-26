// @generated automatically by Diesel CLI.

diesel::table! {
    molecules (id) {
        id -> Int4,
        name -> Text,
        formula -> Text,
        density -> Float8,
        molar_mass -> Float8,
        acentric_factor -> Float8,
        melting_point -> Float8,
        boiling_point -> Float8,
        critical_temperature -> Float8,
        critical_pressure -> Float8,
    }
}
