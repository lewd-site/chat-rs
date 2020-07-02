table! {
    files (id) {
        id -> Int4,
        md5 -> Bpchar,
        size -> Int8,
        name -> Varchar,
        mimetype -> Varchar,
        extension -> Varchar,
        created_at -> Timestamp,
        post_id -> Int4,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        length -> Nullable<Int4>,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        post_id -> Int4,
        user_uuid -> Bpchar,
        read -> Bool,
    }
}

table! {
    posts (id) {
        id -> Int4,
        name -> Varchar,
        tripcode -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        user_uuid -> Nullable<Bpchar>,
    }
}

table! {
    user_favorite_files (id) {
        id -> Int4,
        file_id -> Int4,
        user_uuid -> Bpchar,
    }
}

joinable!(files -> posts (post_id));
joinable!(notifications -> posts (post_id));
joinable!(user_favorite_files -> files (file_id));

allow_tables_to_appear_in_same_query!(
    files,
    notifications,
    posts,
    user_favorite_files,
);
