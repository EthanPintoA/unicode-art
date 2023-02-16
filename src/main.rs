mod ascii_art;

use ascii_art::image_to_ascii;

fn main() {
	let img = image::open("image.png").unwrap();

	let width = 100;
	let ratio = img.height() as f32 / img.width() as f32;
	let height = ((width as f32 * ratio) / 2.0).round() as u32;

	let ascii_art = image_to_ascii(img, width, height);

	println!("{}", ascii_art)
}
