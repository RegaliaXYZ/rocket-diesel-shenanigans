// @generated automatically by Diesel CLI.

diesel::table! {
    engines (id) {
        id -> Int4,
        #[max_length = 50]
        engine_name -> Varchar,
        #[max_length = 50]
        tenant -> Varchar,
        #[max_length = 50]
        engine_type -> Varchar,
        num_utterances -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    engines,
    posts,
);
