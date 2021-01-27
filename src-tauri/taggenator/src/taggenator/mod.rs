// TODO: remove
#![allow(dead_code, warnings, unused)]

mod taggenator;
pub use self::taggenator::Taggenator;

mod settings;

mod utils;
pub use utils::flags;

pub mod errors;
pub use errors::BError;

pub mod database;

pub mod query_processor;

mod commands;
pub use commands::open_all::open_all_core;

pub mod models;

pub static SETTINGS_FILENAME: &str = "tsettings.yaml";

mod tag_recommender;
