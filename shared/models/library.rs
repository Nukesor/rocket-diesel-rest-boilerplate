use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use super::types::MediaType;
use super::schema::libraries;


#[table_name = "libraries"]
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
pub struct Library {
    pub name: String,
    pub media_type: MediaType,
    pub path: String,
}

impl Library {
    pub fn create(hero: Library, connection: &PgConnection) -> Library {
        diesel::insert_into(libraries::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new library");

        libraries::table.order(libraries::name.desc()).first(connection).unwrap()
    }

    pub fn update(name: String, hero: Library, connection: &PgConnection) -> bool {
        diesel::update(libraries::table.find(name)).set(&hero).execute(connection).is_ok()
    }

    pub fn delete(name: String, connection: &PgConnection) -> bool {
        diesel::delete(libraries::table.find(name)).execute(connection).is_ok()
    }
}
