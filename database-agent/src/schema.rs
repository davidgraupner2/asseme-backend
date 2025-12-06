diesel::table! {
    connection_strings (id) {
        id -> Integer,
        value -> Text,
        description -> Nullable<Text>,
        source -> Text,
        status -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}
