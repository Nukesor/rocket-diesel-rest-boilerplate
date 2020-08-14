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
