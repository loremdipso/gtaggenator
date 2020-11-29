use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;

// struct Action {
// 	name: String,
// 	synonyms: Vec<String>,
// 	compute: &'static dyn Fn(&QueryProcessor, Vec<String>, &Database) -> Result<(), BError>,
// 	description: descriptionStruct,
// }

// struct descriptionStruct {
// 	text: String,
// 	// color: String
// 	// color: func(string, ...interface{}) string
// }

// struct QueryProcessor {
// 	myOpener: Opener,
// }

// var actions = []actionStruct{
// 	{"help", []string{"help", "?", "h", "-help", "-h"}, TODO, descriptionStruct{"[action] get help", color.HiBlueString}},
// 	{"open", nil, open, descriptionStruct{"", color.HiBlueString}},
// 	{"open_read_only", nil, open_read_only, descriptionStruct{"Open read only", color.HiBlueString}},
// 	{"open_all", nil, open_all, descriptionStruct{"Open all", color.HiBlueString}},
// 	{"apply_tags", nil, apply_tags, descriptionStruct{"Apply tags (--tag) to the search results. Optionally can use --threads \"#\" to spawn extra workers", color.HiBlueString}},
// 	{"move", nil, move, descriptionStruct{"move results to -destination", color.HiYellowString}},
// 	{"delete", nil, delete, descriptionStruct{"delete results", color.HiYellowString}},
// 	{"dump_tags", nil, dump_tags, descriptionStruct{"Dump all tags", color.HiBlueString}},
// 	{"dump", nil, dump, descriptionStruct{"Dump paths to all entries", color.HiBlueString}},
// 	{"fix", nil, fix, descriptionStruct{"try and fix innacuracies in database", color.HiGreenString}},

// 	// TODO: remove
// 	{"combine", nil, absorb_old_database, descriptionStruct{"[filename] Combine old database", color.HiBlueString}},
// }
