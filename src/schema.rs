table! {
    fs_change_log (id) {
        id -> Integer,
        event_type -> Text,
        file_name -> Text,
        new_name -> Nullable<Text>,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
        notified_at -> Timestamp,
        size -> Integer,
    }
}
