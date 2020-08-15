#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use anyhow::{bail, Result};

use movie::settings::Settings;

mod db;
mod web;

use crate::web::get_rocket;

#[rocket::main]
async fn main() -> Result<()> {
    let settings = Settings::new()?;
    match settings.save() {
        Err(error) => {
            bail!(error.context("Failed saving the config file"));
        }
        Ok(()) => {}
    };

    let rocket = get_rocket(&settings)?;
    rocket.launch().await?;

    Ok(())
}
