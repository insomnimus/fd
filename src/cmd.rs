use std::{
	sync::mpsc,
	thread,
};

use ignore::{
	WalkBuilder,
	WalkState,
};

use crate::app::*;

impl Cmd {
	pub fn run(self) -> i32 {
		let walker = WalkBuilder::new(&self.root)
			.max_depth(self.depth)
			.follow_links(self.follow_links)
			.ignore(self.ignore)
			.parents(false)
			.git_global(false)
			.git_ignore(self.ignore)
			.git_exclude(self.ignore)
			.require_git(true)
			.hidden(!self.hidden)
			.build_parallel();

		let (tx, rx) = mpsc::channel();

		thread::spawn(move || {
			walker.run(move || {
				let tx = tx.clone();

				Box::new(move |res| {
					match res.and_then(|entry| entry.metadata().map(|md| (entry, md))) {
						Ok((entry, md)) if self.file_type.is_match(&md) => {
							#[allow(clippy::question_mark)]
							if tx.send(entry).is_err() {
								return WalkState::Quit;
							}
						}
						Err(e) if !self.quiet => eprintln!("error: {}", e),
						_ => (),
					};
					WalkState::Continue
				})
			});
		});

		let mut results = (0..self.args.len()).map(|_| 0).collect::<Vec<_>>();

		for entry in rx.into_iter() {
			let mut found = false;
			let mut done = true;
			for (i, p) in self.args.iter().enumerate() {
				if self.n == 0 || results[i] < self.n {
					done = false;
					if p.is_match(entry.path()) {
						results[i] += 1;
						found = true;
					}
				}
			}

			if found {
				println!("{}", entry.path().display());
			}
			if done {
				break;
			}
		}
		let mut n_not_found = 0;

		for pattern in
			self.args
				.iter()
				.zip(results)
				.filter_map(|(pattern, n)| if n == 0 { Some(pattern) } else { None })
		{
			n_not_found += 1;
			eprintln!("{}: not found", pattern);
		}
		n_not_found
	}
}
