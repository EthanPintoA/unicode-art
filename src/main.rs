mod ascii_art;

use std::path::PathBuf;

use ascii_art::{image_to_ascii, open_image};
use clap::{arg, value_parser, Command};

fn main() {
	let matches = Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about("Generates ascii art")
		.arg(arg!([FILE] "Image file").value_parser(value_parser!(PathBuf)))
		.arg(arg!(-w --width <width> "Width of ascii art").value_parser(value_parser!(u32)))
		.get_matches();

	// Get args
	let img_path = match matches.get_one::<PathBuf>("FILE") {
		Some(p) => p,
		None => {
			eprintln!("FILE should not be None");
			return;
		},
	};

	let width = *matches.get_one::<u32>("width").unwrap_or(&100);

	let img = match open_image(img_path) {
		Ok(i) => i,
		Err(e) => {
			eprintln!("{}", e);
			return;
		},
	};

	let ratio = img.height() as f32 / img.width() as f32;
	let height = ((width as f32 * ratio) / 2.0).round() as u32;

	let ascii_art = image_to_ascii(img, width, height);

	println!("{}", ascii_art)
}
