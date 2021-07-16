// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

mod args;
mod buffer;
mod engine;
mod errorlist;
mod counter;
mod input;
mod matcher;
mod printer;

fn main() {
	use clap::Clap;
	let mut opts = args::Args::parse();
	let linesources = exit_if_err(opts.build_linesources());
	let has_multiple = linesources.len() > 1;
	let should_prefix = opts.should_prefix_lines().unwrap_or(has_multiple);
	let mut engine = exit_if_err(engine::Engine::new(opts));
	for linesrc in linesources {
		engine.process(linesrc, should_prefix)
	}
}

fn exit_if_err<T>(result: Result<T,errorlist::ErrorList>) -> T {
	match result {
		Ok(t)  => t,
		Err(e) => e.print_all_and_exit()
	}
}
