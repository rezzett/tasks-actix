table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        content -> Text,
        category_id -> Integer,
    }
}

joinable!(tasks -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    categories,
    tasks,
);
