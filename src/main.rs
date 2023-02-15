use image::imageops::FilterType;

fn main() {
	let img = image::open("image.png").unwrap();

	let img = img.resize_exact(100, 50, FilterType::Nearest);
	let _image_buf = img.to_luma8();
}
