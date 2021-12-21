mod app;
mod cmd;
mod pattern;

fn main() {
	std::process::exit(match app::Cmd::from_args() {
		Ok(c) => c.run(),
		Err(e) => {
			eprintln!("error: {}", e);
			1
		}
	})
}
