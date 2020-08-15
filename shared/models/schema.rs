table! {
    use diesel::sql_types::*;
    use crate::models::types::*;

    libraries (name) {
        name -> Text,
        media_type -> Media_type,
        path -> Text,
        location -> Library_location,
    }
}
