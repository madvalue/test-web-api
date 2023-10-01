// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Integer,
        #[max_length = 255]
        nickname -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        author_id -> Nullable<Integer>,
    }
}

diesel::joinable!(posts -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    posts,
);
