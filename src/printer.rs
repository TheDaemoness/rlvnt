// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use std::io::Write;

pub struct Printer(std::io::Stdout);

pub type Closure<'a> = Box<dyn FnMut(&str) + 'a>;

impl Printer {
	pub fn new() -> Printer {
		Printer(std::io::stdout())
	}

	pub fn closure<'a>(&'a mut self) -> Closure<'a> {
		Box::new(move |line| {
			write_line(&mut self.0.lock(), line)
		})
	}

	pub fn closure_with_prefix<'a>(&'a mut self, prefix: &'a str) -> Closure<'a> {
		Box::new(move |line| {
			let mut lock = self.0.lock();
			write(&mut lock, prefix);
			write(&mut lock, ":");
			write_line(&mut lock, line);
		})
	}
}

fn write(out: &mut std::io::StdoutLock, string: &str) {
	let _ = out.write_all(string.as_bytes());
}
fn write_line(out: &mut std::io::StdoutLock, line: &str) {
	write(out, line);
	write(out, "\n");
}

