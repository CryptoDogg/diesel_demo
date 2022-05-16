table! {
    posts (id) {
        id -> Int4,
        time -> Timestamp,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
