use std::ffi::OsStr;
use std::path::Path;

pub fn dedup<T>(list: &mut Vec<T>)
where
	T: std::cmp::PartialEq,
	T: Clone,
{
	let mut to_remove = vec![];
	for (ai, a) in list.iter().enumerate() {
		for (bi, b) in list.iter().skip(ai + 1).enumerate() {
			let bi = bi + ai + 1;
			if a == b {
				// prefer
				if !to_remove.contains(&ai) {
					to_remove.push(ai);
				}
			}
		}
	}

	// remove in reverse order for obvious reasons
	for index in to_remove.iter().rev() {
		list.remove(*index);
	}
}

#[cfg(test)]
mod dedup_tests {
	use crate::taggenator::utils::lists::dedup;

	#[test]
	fn basic() {
		let mut a = vec!["A", "A", "C", "D", "E", "A"];
		dedup(&mut a);
		assert_eq!(a, vec!["C", "D", "E", "A"]);
	}
}
