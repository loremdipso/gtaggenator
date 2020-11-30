use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::Taggenator;

pub fn dump_tags(db: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut searcher = Searcher::new(args)?;

	let result = searcher.get_tags(&db.database)?;
	dbg!(result);

	// if args.len() == 0 {
	// 	// Special Case: no search arguments, just print all tags
	// 	tags, err := getAllTags(db)
	// 	if err != nil {
	// 		return err
	// 	}

	// 	// fmt.Println("Tag Dump:", go_utils.StringArrayToString(tags))
	// 	for _, tag := range tags {
	// 		fmt.Println(tag)
	// 	}
	// } else {
	// 	// NOTE: this assumes vlc
	// 	// TODO: make more generic
	// 	search := searcher.New(db)
	// 	err := search.Parse(args)
	// 	if err != nil {
	// 		return err
	// 	}

	// 	entries, err := search.Execute()
	// 	if err != nil {
	// 		return err
	// 	}

	// 	if len(entries) == 0 {
	// 		color.HiRed("# No entries")
	// 		return nil
	// 	} else {
	// 		fmt.Printf("# Found %s entries\n", color.HiBlueString("%d", len(entries)))
	// 	}

	// 	sortedTags := make([]string, 0, 1000) // TODO: length?
	// 	for _, entry := range entries {
	// 		for _, tag := range entry.Tags {
	// 			_, sortedTags = go_utils.InsertIntoSortedListIfNotThereAlready(sortedTags, tag)
	// 		}
	// 	}

	// 	for _, tag := range sortedTags {
	// 		fmt.Println(tag)
	// 	}
	// }

	return Ok(());
}
