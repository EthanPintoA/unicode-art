use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, Luma};

fn luma_to_char(luma: &Luma<u8>) -> char {
	// http://paulbourke.net/dataformats/asciiart/
	const ASCII_CHARS: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
	const LAST_IDX: f32 = ASCII_CHARS.len() as f32 - 1.0;

	// Converts luma[0]/255 (u8) to idx/LAST_IDX
	let ratio: u8 = (255.0 / LAST_IDX).ceil() as u8;
	ASCII_CHARS[(luma[0] / ratio) as usize]
}

fn preprocess_image(
	image: DynamicImage,
	width: u32,
	height: u32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
	let image = image.resize_exact(width, height, FilterType::Nearest);
	let image_buf = image.to_luma_alpha8();

	ImageBuffer::from_fn(width, height, |x, y| {
		let luma_a = image_buf.get_pixel(x, y);
		// Blend and remove alpha
		// https://en.wikipedia.org/wiki/Alpha_compositing
		[(luma_a[0] as f32 * (luma_a[1] as f32 / 255.0)) as u8].into()
	})
}

fn main() {
	let img = image::open("image.png").unwrap();

	let image_buf = preprocess_image(img, 100, 50);

	let ascii_art = image_buf
		.rows()
		.map(|p| p.map(luma_to_char).collect::<String>())
		.collect::<Vec<_>>()
		.join("\n");

	println!("{}", ascii_art)
}
