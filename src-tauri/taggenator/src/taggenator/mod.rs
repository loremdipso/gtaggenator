// TODO: remove
#![allow(dead_code, warnings, unused)]

mod taggenator;
pub use taggenator::Taggenator;

mod settings;

mod utils;

mod errors;
pub use errors::BError;

pub mod database;

pub mod query_processor;

mod commands;

pub mod models;
