use std::io::Write;

//TODO: This file needs a rename and a restructure.

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
	use_before: bool,
	before: std::collections::VecDeque<String>
}

impl BuffingPrinter {
	pub fn new() -> BuffingPrinter {
		BuffingPrinter{ use_before: false, before: std::collections::VecDeque::new() }
	}
	pub fn push(&mut self, line: String, matched: bool, prefix: &str) {
		if matched {
			if !self.before.is_empty() {
				self.before.drain(0..).for_each(|line| {
					print_line(prefix, &line);
				});
			} else {
				self.use_before = true
			}
			print_line(prefix, &line);
		} else if self.use_before {
			self.before.push_back(line);
		}
	}
}
