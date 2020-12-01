use std::sync::Arc;
use std::sync::Mutex;
use taggenator::errors::MyCustomError::UnknownError;
use taggenator::taggenator::database::searcher::Searcher;
use taggenator::BError;
use taggenator::Taggenator;
use warp::Filter;

mod actions;
mod cmd;
mod file_server;

pub use actions::start_tauri;
