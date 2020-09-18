table! {
    entries (id) {
        id -> Integer,
        title -> Text,
        uploader -> Text,
    }
}

table! {
    tagmap (id) {
        id -> Integer,
        tag_id -> Integer,
        entry_id -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(tagmap -> entries (entry_id));
joinable!(tagmap -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    entries,
    tagmap,
    tags,
    users,
);
