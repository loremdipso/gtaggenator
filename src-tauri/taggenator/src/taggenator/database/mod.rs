use crate::taggenator::errors::MyCustomError;

mod database;
pub mod searcher;
mod writer;

pub static CACHE_FILENAME: &str = ".gtaggenator_cache.yaml";
pub static DATABASE_FILENAME: &str = "tagg.db";

use crate::taggenator::errors::BError;
pub use database::Database;
