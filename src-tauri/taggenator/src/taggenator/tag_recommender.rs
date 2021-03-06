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
			// remove all non-alpha/numba/spacing characters
			re: Regex::new(r#"[^A-Za-z0-9 ]"#).unwrap(),
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
			let temp_tag: String = tag.chars().skip(index).collect::<String>().to_lowercase();
			let temp_tag = self.re.replace_all(&temp_tag, "");
			let temp_tag = temp_tag.to_lowercase().replace(" ", "");
			let entry = self
				.mapping
				.entry(temp_tag.to_string())
				.or_insert(HashSet::new());
			// (*entry).insert(prefix);
			(*entry).insert(tag.to_string());
		} else {
			let temp_tag = self.re.replace_all(&tag, "");
			let temp_tag = temp_tag.to_lowercase().replace(" ", "");
			let entry = self.mapping.entry(temp_tag).or_insert(HashSet::new());
			(*entry).insert(tag.to_lowercase().to_string());
		}
	}

	pub fn recommend_from_location(
		&self,
		location: &String,
		existing_tags: &HashSet<String>,
	) -> HashSet<String> {
		let items: Vec<String> = self
			.re
			.replace_all(&location.to_lowercase(), " ")
			.split(" ")
			.filter(|e| e.len() > 0)
			.map(|e| e.to_string())
			.collect();

		return self.helper(items, existing_tags);
	}

	pub fn recommend_from_grabbag(
		&self,
		grabbag_tags: Vec<String>,
		existing_tags: &HashSet<String>,
	) -> HashSet<String> {
		return self.helper(grabbag_tags, existing_tags);
	}

	pub fn helper(&self, items: Vec<String>, existing_tags: &HashSet<String>) -> HashSet<String> {
		let mut rv = HashSet::new();
		let max_count = std::cmp::min(items.len(), 5);
		for start in (0..items.len()) {
			for end in ((start + 1)..=(std::cmp::min(items.len(), start + max_count))) {
				let range = &items[start..end];

				// we'll join without spaces since we carefully avoided spaces in the key above
				let tag = range.join("");
				if let Some(matches) = self.mapping.get(&tag) {
					for some_match in matches {
						if existing_tags.iter().find(|el| el == &some_match).is_none() {
							rv.insert(some_match.to_string());
						}
					}
				}
			}
		}

		return rv;
	}
}

#[cfg(test)]
mod flag_tests {
	use std::collections::HashSet;

	use super::TagRecommender;

	#[test]
	fn basic() {
		let all_tags = vec![
			"some:prefix:a".to_string(),
			"prefix:a".to_string(),
			"prefix:a b".to_string(),
			"prefix:a b c".to_string(),
			"prefix:H-i".to_string(),
			"prefix:jk".to_string(),
			"not a match".to_string(),
		];

		let mut existing_tags = HashSet::new();
		existing_tags.insert("prefix:a b".to_string());

		let location = "[H-i] a b c d e-f[]/g.\\j k".to_string();

		let mut correct_answer = vec![
			"prefix:a".to_string(),
			"some:prefix:a".to_string(),
			"prefix:a b c".to_string(),
			"prefix:H-i".to_string(),
			"prefix:jk".to_string(),
		];

		let recommender = TagRecommender::new(all_tags.iter());
		let mut answer = recommender.recommend_from_location(&location, &existing_tags);

		assert_eq!(answer.len(), correct_answer.len());
		for tag in correct_answer {
			assert_eq!(answer.contains(&tag), true);
		}
	}

	#[test]
	fn extended() {
		let all_tags = vec!["beginning".to_string(), "ending".to_string()];

		let mut existing_tags = HashSet::new();

		let location = "beginning &#39;s ending.ext".to_string();

		let mut correct_answer = vec!["beginning".to_string(), "ending".to_string()];

		let recommender = TagRecommender::new(all_tags.iter());
		let mut answer = recommender.recommend_from_location(&location, &existing_tags);

		assert_eq!(answer.len(), correct_answer.len());
		for tag in correct_answer {
			assert_eq!(answer.contains(&tag), true);
		}
	}
}
