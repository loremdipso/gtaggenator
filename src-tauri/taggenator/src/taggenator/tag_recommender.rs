use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct TagRecommender {
	mapping: HashMap<String, HashSet<String>>,
	re: Regex,
}

impl<'a> TagRecommender {
	pub fn new<T: Iterator<Item = &'a String>>(tags: T) -> TagRecommender {
		let mut mapping: HashMap<String, HashSet<String>> = HashMap::new();
		let mut rv = TagRecommender {
			mapping,
			re: Regex::new(r#"[-_/\\.]"#).unwrap(),
		};

		rv.add_tags(tags);

		return rv;
	}

	pub fn add_tags<T: Iterator<Item = &'a String>>(&mut self, tags: T) {
		for tag in tags {
			// find last ':'
			self.add_tag(tag);
		}
	}

	pub fn add_tag(&mut self, tag: &String) {
		let index = tag.chars().rev().position(|c| c == ':');

		if let Some(index) = index {
			let index = tag.chars().count() - index;

			// let prefix: String = tag.chars().take(index - 1).collect();
			let temp_tag: String = tag.chars().skip(index).collect();

			let entry = self
				.mapping
				.entry(temp_tag.to_lowercase().to_string())
				.or_insert(HashSet::new());
			// (*entry).insert(prefix);
			(*entry).insert(tag.to_string());
		} else {
			let entry = self
				.mapping
				.entry(tag.to_string())
				.or_insert(HashSet::new());
			(*entry).insert(tag.to_lowercase().to_string());
		}
	}

	pub fn recommend(&self, location: &String, existing_tags: &HashSet<String>) -> Vec<String> {
		let mut rv = vec![];

		let items: Vec<String> = self
			.re
			.replace_all(&location.to_lowercase(), " ")
			.split(" ")
			.filter(|e| e.len() > 0)
			.map(|e| e.to_string())
			.collect();

		let max_count = std::cmp::min(items.len(), 5);
		for start in (0..items.len()) {
			for end in ((start + 1)..=(std::cmp::min(items.len(), start + max_count))) {
				let range = &items[start..end];
				let tag = range.join(" ");
				if let Some(matches) = self.mapping.get(&tag) {
					for some_match in matches {
						if existing_tags.iter().find(|el| el == &some_match).is_none() {
							rv.push(some_match.to_string());
						}
					}
				}
			}
		}

		return rv;
	}
}

#[test]
fn test_recommend_tags() {
	let all_tags = vec![
		"some:prefix:a".to_string(),
		"prefix:a".to_string(),
		"prefix:a b".to_string(),
		"prefix:a b c".to_string(),
		"not a match".to_string(),
	];

	let mut existing_tags = HashSet::new();
	existing_tags.insert("prefix:a b".to_string());

	let location = "a b c d e-f/g.\\h_i".to_string();

	let mut correct_answer = vec![
		"prefix:a".to_string(),
		"some:prefix:a".to_string(),
		"prefix:a b c".to_string(),
	];
	correct_answer.sort();

	let recommender = TagRecommender::new(all_tags.iter());
	let mut answer = recommender.recommend(&location, &existing_tags);
	answer.sort();

	assert_eq!(answer, correct_answer);
}
