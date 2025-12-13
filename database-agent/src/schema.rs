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
    properties (id) {
        id -> Integer,
        key -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        description -> Nullable<Text>,
        value_int -> Nullable<Integer>,
        value_string -> Nullable<Text>,
        value_bool -> Nullable<Integer>,
        value_json -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::allow_tables_to_appear_in_same_query!(
    connection_strings,
    events,
    function_hashes,
    properties,
    tags,
);
