use crate::taggenator::errors::MyCustomError;

mod database;
pub mod searcher;
mod writer;

static SETTINGS_FILENAME: &str = "tagg.db";
static END_OF_WRITES: &str = "end";

use crate::taggenator::errors::BError;
pub use database::Database;
