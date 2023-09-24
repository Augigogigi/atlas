use once_cell::unsync::Lazy;

/* Global Framebuffer */
pub static mut FRAMEBUFFER: Lazy<Framebuffer> = Lazy::new(|| unsafe { Framebuffer(crate::boot::KERNEL_DATA.framebuffer.unwrap().as_ptr()) });

pub struct Framebuffer(*mut limine::Framebuffer);

impl Framebuffer {
	#[inline(always)]
	fn draw_pixel_raw(&mut self, x: usize, y: usize, color: u32) {
		unsafe {
			*(self.address.as_ptr().unwrap().offset(((y * self.pitch as usize) + (x * 4)) as isize) as *mut u32) = color;
		}
	}

	/* Clear the screen */
	pub fn clear(&mut self, color: u32) {
		for y in 0..self.height as usize {
			for x in 0..self.width as usize {
				self.draw_pixel_raw(x, y, color);
			}
		}
	}

	/* Draw a pixel to the screen */
	pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
		self.draw_pixel_raw(x.min(self.width as usize), y.min(self.height as usize), color);
	}

	/* Draw a line to the screen */
	pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
		let mut x1 = x1.min(self.width as usize) as isize;
		let mut y1 = y1.min(self.height as usize) as isize;
		let x2 = x2.min(self.width as usize) as isize;
		let y2 = y2.min(self.height as usize) as isize;

		let dx = (x2 as isize - x1 as isize).abs();
		let dy = (y2 as isize - y1 as isize).abs() * -1;
		let sx = if x1 < x2 { 1 } else { -1 };
		let sy = if y1 < y2 { 1 } else { -1 };
		let mut error = dx + dy;

		while x1 != x2 || y1 != y2 {
			self.draw_pixel_raw(x1 as usize, y1 as usize, color);
			let e2 = error * 2;
			if e2 >= dy {
				if x1 == x2 {
					break;
				}
				error += dy;
				x1 = x1 as isize + sx;
			}
			if e2 <= dx {
				if y1 == y2 {
					break;
				}
				error += dx;
				y1 = y1 as isize + sy;
			}
		}
	}

	/* Draw a rectangle to the screen */
	pub fn draw_rect(&mut self, x: usize, y: usize, x2: usize, y2: usize, color: u32) {
		self.draw_line(x, y, x2, y, color);
		self.draw_line(x2, y, x2, y2, color);
		self.draw_line(x2, y2, x, y2, color);
		self.draw_line(x, y2, x, y, color);
	}
}

impl core::ops::Deref for Framebuffer {
    type Target = limine::Framebuffer;

    fn deref(&self) -> &Self::Target {
		unsafe {
			&*self.0
		}
    }
}
impl core::ops::DerefMut for Framebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			&mut *self.0
		}
    }
}
