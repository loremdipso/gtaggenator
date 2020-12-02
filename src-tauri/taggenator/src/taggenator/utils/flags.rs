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
