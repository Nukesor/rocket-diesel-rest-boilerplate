#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use std::collections::HashMap;
use anyhow::{bail, Result};
use rocket::config::{Config, Environment};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use movie::models::Library;
use movie::settings::Settings;

mod db;

use db::Db;

#[post("/", data = "<library>")]
fn create(library: Json<Library>, db: Db) -> Json<Library> {
    let insert = Library { ..library.into_inner() };
    Json(Library::create(insert, &db))
}

#[put("/<name>", data = "<library>")]
fn update(name: String, library: Json<Library>, db: Db) -> JsonValue {
    let update = Library { name: name.clone(), ..library.into_inner() };
    json!({
        "success": Library::update(name, update, &db)
    })
}

#[delete("/<name>")]
fn delete(name: String, db: Db) -> JsonValue  {
    json!({
        "success": Library::delete(name, &db)
    })
}

#[rocket::main]
async fn main() -> Result<()> {
    let settings = Settings::new()?;
    match settings.save() {
        Err(error) => {
            bail!(error.context("Failed saving the config file"));
        }
        Ok(()) => {}
    };


    let mut database_config = HashMap::new();
    database_config.insert("url", settings.server.database_uri);
    let mut databases = HashMap::new();
    databases.insert("my_db", database_config);

    let config = Config::build(Environment::Production)
        .extra("databases", databases)
        .address("0.0.0.0")
        .port(8456)
        .finalize()?;

    rocket::custom(config)
        .mount("/library", routes![create, update, delete])
        .launch()
        .await?;

    Ok(())
}
