use std::collections::HashMap;

use anyhow::Result;
use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_contrib::databases::{database, diesel};

use movie::settings::Settings;

mod library;

#[database("movie")]
pub struct Db(diesel::PgConnection);

pub fn get_rocket(settings: &Settings) -> Result<Rocket> {
    let mut database_config = HashMap::new();
    database_config.insert("url", settings.server.database_uri.clone());

    let mut databases = HashMap::new();
    databases.insert("my_db", database_config);

    let config = Config::build(Environment::Production)
        .extra("databases", databases)
        .address("0.0.0.0")
        .port(8456)
        .finalize()?;

    Ok(rocket::custom(config).mount(
        "/library",
        routes![
            library::create,
            library::delete,
            library::get,
            library::update
        ],
    ))
}
