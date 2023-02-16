use image::imageops::FilterType;
use image::Luma;

fn luma_to_char(luma: &Luma<u8>) -> char {
	// http://paulbourke.net/dataformats/asciiart/
	const ASCII_CHARS: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
	const LAST_IDX: f32 = ASCII_CHARS.len() as f32 - 1.0;

	// Converts luma[0]/255 (u8) to idx/LAST_IDX
	let ratio: u8 = (255.0 / LAST_IDX).ceil() as u8;
	ASCII_CHARS[(luma[0] / ratio) as usize]
}

fn main() {
	let img = image::open("image.png").unwrap();

	let img = img.resize_exact(100, 50, FilterType::Nearest);
	let image_buf = img.to_luma8();

	let ascii_art = image_buf
		.rows()
		.map(|p| p.map(luma_to_char).collect::<String>())
		.collect::<Vec<_>>()
		.join("\n");

	println!("{}", ascii_art)
}
