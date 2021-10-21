mod app;
mod cmd;

fn main() {
	if !cmd::Cmd::from_args().run() {
		std::process::exit(3);
	}
}
