mod ascii_art;

use std::path::PathBuf;

use ascii_art::image_to_ascii;
use clap::{arg, value_parser, Command};

fn main() {
	let matches = Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about("Generates ascii art")
		.arg(arg!([FILE] "Image file").value_parser(value_parser!(PathBuf)))
		.arg(arg!(-w --width <width> "Width of ascii art").value_parser(value_parser!(u32)))
		.get_matches();

	let img_path = matches.get_one::<PathBuf>("FILE").unwrap();
	let width = *matches.get_one::<u32>("width").unwrap_or(&100);

	let img = image::open(img_path).unwrap();

	let ratio = img.height() as f32 / img.width() as f32;
	let height = ((width as f32 * ratio) / 2.0).round() as u32;

	let ascii_art = image_to_ascii(img, width, height);

	println!("{}", ascii_art)
}
