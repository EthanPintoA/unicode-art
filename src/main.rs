mod ascii_art;

use ascii_art::image_to_ascii;

fn main() {
	let img = image::open("image.png").unwrap();

	let ascii_art = image_to_ascii(img, 100, 50);

	println!("{}", ascii_art)
}
