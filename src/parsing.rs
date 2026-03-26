

pub fn parse_command(user_input: &str) -> Vec<&str> {
	let user_input = user_input.trim();
	if user_input.starts_with('/') {
		let args: Vec<&str> = user_input.split_whitespace().collect();
		let cmd: &str = args[0];
		//println!("{}, {:?}", cmd, &args[1..]);

		return args;
	}
	return Vec::new();
}