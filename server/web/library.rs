use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use movie::models::Library;

use super::Db;

#[post("/", data = "<library>")]
pub fn create(library: Json<Library>, db: Db) -> Json<Library> {
    let insert = Library {
        ..library.into_inner()
    };
    Json(Library::create(insert, &db))
}

#[put("/<name>", data = "<library>")]
pub fn update(name: String, library: Json<Library>, db: Db) -> JsonValue {
    let update = Library {
        name: name.clone(),
        ..library.into_inner()
    };
    json!({ "success": Library::update(name, update, &db) })
}

#[delete("/<name>")]
pub fn delete(name: String, db: Db) -> JsonValue {
    json!({ "success": Library::delete(name, &db) })
}
