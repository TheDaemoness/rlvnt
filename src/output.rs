use std::io::Write;

//TODO: This file will be removed soon.

#[allow(unused_must_use)]
pub fn print_line(prefix: &str, line: &str) {
	//println! panics on a broken pipe. We want silent failure.
	if !prefix.is_empty() {
		std::io::stdout().write_all(prefix.as_bytes());
		std::io::stdout().write_all(":".as_bytes());
	}
	std::io::stdout().write_all(line.as_bytes());
	std::io::stdout().write_all("\n".as_bytes());
}

/** Storing and printing lines.*/
pub struct BuffingPrinter {
	before: std::collections::VecDeque<String>
}

impl BuffingPrinter {
	pub fn new() -> BuffingPrinter {
		BuffingPrinter{ before: std::collections::VecDeque::new() }
	}
	pub fn print_all(&mut self, line: String, prefix: &str) {
		self.before.drain(0..).for_each(|buffed_line| {
			print_line(prefix, &buffed_line);
		});
		print_line(prefix, &line);
	}
	pub fn push(&mut self, line: String) {
		self.before.push_back(line);
	}
}
