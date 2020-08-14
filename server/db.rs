use rocket_contrib::databases::{database, diesel};

#[database("movie")]
pub struct Db(diesel::PgConnection);
