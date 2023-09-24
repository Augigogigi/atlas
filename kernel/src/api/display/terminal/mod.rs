pub mod font;
pub mod macros;

use once_cell::unsync::Lazy;

const MARGIN: (usize, usize) = (4, 4);

/* Global Terminal */
pub static mut TERMINAL: Lazy<Terminal> = Lazy::new(|| unsafe {
	super::framebuffer::FRAMEBUFFER.clear(super::BG_COLOR);
	Terminal {
		framebuffer: &mut super::framebuffer::FRAMEBUFFER,
		font: font::DEFAULT_FONT,
		size: ((super::framebuffer::FRAMEBUFFER.width as usize - (MARGIN.0 * 2)) / font::DEFAULT_FONT.char_width, (super::framebuffer::FRAMEBUFFER.height as usize - (MARGIN.0 * 2)) / font::DEFAULT_FONT.char_height),
		margin: MARGIN,
		cursor: (0, 0),
		color: super::TEXT_COLOR,
	}
});

pub struct Terminal<'a> {
	framebuffer: &'a mut Lazy<super::framebuffer::Framebuffer>,
	font: Lazy<font::Font>,
	size: (usize, usize),
	margin: (usize, usize),
	cursor: (usize, usize),
	color: u32,
}

impl<'a> Terminal<'a> {
	/* Reset the terminal */
	pub fn reset(&mut self) {
		self.framebuffer.clear(super::BG_COLOR);
		self.cursor.0 = 0;
		self.cursor.1 = 0;
	}

	/* Go to a new line */
	pub fn newline(&mut self) {
		self.cursor.0 = 0;
		self.cursor.1 += 1;
	}

	pub fn color(&mut self, color: u32) {
		self.color = color;
	}

	/* Write a character */
	fn write_char(&mut self, character: char) {
		if self.cursor.0 >= self.size.0 {
			self.newline();
		}

		if self.cursor.1 >= self.size.1 {
			self.reset();
		}

		match character {
			' '..='~' => {
				// WARNING: ASSUMES 8-BIT WIDTH (will fix later)
				let mut glyph: [u8;16] = [0;16];
				glyph.copy_from_slice(&self.font.data[(character as usize * 16)..(character as usize * 16 + 16)]);

				for row in 0..self.font.char_height {
					for col in 0..self.font.char_width {
						if (1 << col) & glyph[row] > 0 {
							let margin_offset_x = unsafe { super::framebuffer::FRAMEBUFFER.width as usize % font::DEFAULT_FONT.char_width >> 1 };
							let margin_offset_y = unsafe { super::framebuffer::FRAMEBUFFER.height as usize % font::DEFAULT_FONT.char_height >> 1 };
							let cursor_offset_x = (self.cursor.0 * self.font.char_width) + margin_offset_x + self.margin.0;
							let cursor_offset_y = (self.cursor.1 * self.font.char_height) + margin_offset_y + self.margin.1;
		
							self.framebuffer.draw_pixel(cursor_offset_x + (7 - col), cursor_offset_y + row, self.color);
						}
					}
				}
				
				self.cursor.0 += 1;
			},
			'\n' => self.newline(),
			_ => {},
		}
	}
}

impl<'a> core::fmt::Write for Terminal<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for character in s.chars() {
			self.write_char(character);
		}
		Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
	use core::fmt::Write;
	unsafe {
		TERMINAL.write_fmt(args).unwrap();
	}
}