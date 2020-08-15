use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use super::schema::libraries;
use super::types::{LibraryLocation, MediaType};

#[table_name = "libraries"]
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
pub struct Library {
    pub name: String,
    pub media_type: MediaType,
    pub path: String,
    pub location: LibraryLocation,
}

impl Library {
    pub fn get_all(db: &PgConnection) -> Vec<Library> {
        libraries::table
            .order(libraries::name.desc())
            .load::<Library>(db)
            .unwrap()
    }

    pub fn create(library: Library, db: &PgConnection) -> Library {
        diesel::insert_into(libraries::table)
            .values(&library)
            .execute(db)
            .expect("Error creating new library");

        libraries::table
            .order(libraries::name.desc())
            .first(db)
            .unwrap()
    }

    pub fn update(name: String, library: Library, db: &PgConnection) -> bool {
        diesel::update(libraries::table.find(name))
            .set(&library)
            .execute(db)
            .is_ok()
    }

    pub fn delete(name: String, db: &PgConnection) -> bool {
        diesel::delete(libraries::table.find(name))
            .execute(db)
            .is_ok()
    }
}
