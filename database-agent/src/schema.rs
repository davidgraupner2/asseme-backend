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
    events (id) {
        id -> Integer,
        event_type -> Text,
        aggregate_type -> Text,
        aggregate_id -> Text,
        payload -> Text,
        metadata -> Nullable<Text>,
        status -> Text,
        retry_count -> Integer,
        processed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    function_hashes (id) {
        id -> Integer,
        function_hash -> Text,
        description -> Nullable<Text>,
        source -> Text,
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
