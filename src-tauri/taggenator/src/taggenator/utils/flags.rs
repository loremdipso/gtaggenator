pub fn take_flag(args: &mut Vec<String>, flag: &str) -> bool {
	let mut rv = false;
	args.retain(|arg| {
		let keep = (arg != flag);
		if !keep {
			rv = true;
		}
		return keep;
	});
	return rv;
}

pub fn take_flag_with_arg(args: &mut Vec<String>, flag: &str) -> Option<String> {
	for i in 0..(args.len()) {
		let arg = &args[i];
		if arg == flag {
			let next = &args[i + 1];
			let rv = Some(next.to_string());
			args.remove(i + 1);
			args.remove(i);
			return rv;
		}
	}

	return None;
}

#[cfg(test)]
mod flag_tests {
	use crate::flags::take_flag_with_arg;

	#[test]
	fn found() {
		let mut args = vec!["A".to_string(), "B".to_string(), "C".to_string()];
		let flag = take_flag_with_arg(&mut args, "B");
		assert_eq!(flag, Some("C".to_string()));
	}

	#[test]
	fn no_arg() {
		let mut args = vec!["A".to_string(), "B".to_string(), "C".to_string()];
		let flag = take_flag_with_arg(&mut args, "C");
		assert_eq!(flag, None);
	}

	#[test]
	fn no_flag() {
		let mut args = vec!["A".to_string(), "B".to_string(), "C".to_string()];
		let flag = take_flag_with_arg(&mut args, "D");
		assert_eq!(flag, None);
	}
}
