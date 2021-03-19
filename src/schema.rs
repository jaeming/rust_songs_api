table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    songs (id) {
        id -> Int4,
        title -> Varchar,
        duration -> Int4,
        artist_id -> Int4,
    }
}

joinable!(songs -> artists (artist_id));

allow_tables_to_appear_in_same_query!(
    artists,
    songs,
);
