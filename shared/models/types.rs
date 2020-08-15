use diesel_derive_enum::DbEnum;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, DbEnum)]
#[PgType = "media_type"]
#[DieselType = "Media_type"]
pub enum MediaType {
    Show,
    Movie,
    Clip,
}

#[derive(Clone, Debug, Deserialize, Serialize, DbEnum)]
#[PgType = "library_location"]
#[DieselType = "Library_location"]
pub enum LibraryLocation {
    Local,
    SSH,
}
