use std::process::{Command, Stdio};
use crate::config;

pub fn open_search(config: &config::Config, entries: &mut Vec<String>) -> String {
	let entries_string: String = entries
		.iter_mut()
		.reduce(|acc: &mut String, s: &mut String| {acc.push_str("\n"); acc.push_str(s); acc})
		.expect("failed to get dmenu list").to_string();

	let echo_child = Command::new("echo")
			.arg("-ne")
			.arg(&entries_string)
			.stdout(Stdio::piped())
			.spawn()
			.expect("failed to execute the dmenu");

	let echo_out = echo_child.stdout.expect("failed to execute echo");

	let dmenu_command_iter: Vec<&str> = config.dmenu_command.split(" ").collect();

	let dmenu_child = Command::new(dmenu_command_iter[0])
		.arg(dmenu_command_iter[1])
		.stdin(Stdio::from(echo_out))
		.spawn()
		.expect("failed to execute the dmenu");

	let output = dmenu_child.wait_with_output().expect("hello");
	println!("{}", String::from_utf8(output.stdout.clone()).unwrap());
	String::from_utf8(output.stdout).unwrap()
}
