mod database;
mod writer;

static SETTINGS_FILENAME: &str = "tagg.db";
static END_OF_WRITES: &str = "end";

pub use database::Database;
// use rusqlite::ToSql;

// struct Query<P>
// where
// 	P: IntoIterator,
// 	P::Item: ToSql,
// {
// 	sql: String,
// 	params: P,
// }
