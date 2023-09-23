use once_cell::unsync::Lazy;

pub struct Font {
	pub char_width: usize,
	pub char_height: usize,
	pub char_count: usize,
	pub data: &'static [u8],
}

pub const DEFAULT_FONT: Lazy<Font> = Lazy::new(|| Font {
	char_width: 8,
	char_height: 16,
	char_count: 256,
	data: &include_bytes!("zap-light16.psf")[4..],
});