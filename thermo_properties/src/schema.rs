// @generated automatically by Diesel CLI.

diesel::table! {
    antoine_coeff (id) {
        id -> Int4,
        molecule_id -> Int4,
        min_temp -> Float8,
        max_temp -> Float8,
        a -> Float8,
        b -> Float8,
        c -> Float8,
    }
}

diesel::table! {
    molecules (molecule_id) {
        molecule_id -> Int4,
        name -> Text,
        formula -> Text,
        density -> Nullable<Float8>,
        molar_mass -> Nullable<Float8>,
        acentric_factor -> Nullable<Float8>,
        melting_point -> Nullable<Float8>,
        boiling_point -> Nullable<Float8>,
        critical_temperature -> Nullable<Float8>,
        critical_pressure -> Nullable<Float8>,
    }
}

diesel::joinable!(antoine_coeff -> molecules (molecule_id));

diesel::allow_tables_to_appear_in_same_query!(
    antoine_coeff,
    molecules,
);
