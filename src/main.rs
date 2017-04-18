extern crate libsrl;

pub mod app;
pub mod keys;
pub mod render;
pub mod handle;

use app::App;

use std::env;

fn main() {
	let args : Vec<_> = env::args().collect();
	if args.len() != 2 {
		println!("usage: srl_ui <filename>");
		return;
	}
	let mut app = match App::by_filename(&args[1]) {
		Ok(x) => x,
		Err(srl_error) => {
			println!("{}", srl_error.to_string());
			return;
		}
	};
	app.run();
}
