use std::io::Write;

#[allow(unused_must_use)]
pub fn print_line(line: &str) {
	//println! panics on a broken pipe. We want silent failure.
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
	pub fn push(&mut self, line: String, matched: bool) {
		if matched {
			if !self.before.is_empty() {
				self.before.drain(0..).for_each(|line| {
					print_line(&line);
				});
			} else {
				self.use_before = true
			}
			print_line(&line);
		} else if self.use_before {
			self.before.push_back(line);
		}
	}
}
