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
    posts (id) {
        id -> Int4,
        name -> Varchar,
        tripcode -> Varchar,
        message -> Text,
        created_at -> Timestamp,
    }
}

joinable!(files -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    files,
    posts,
);
