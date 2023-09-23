#![no_std]
#![no_main]

#![feature(strict_provenance)]
#![feature(const_mut_refs)]

pub mod api;
pub mod arch;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
	kprintln!("Hello, World!");

	hang();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	unsafe {
		use api::display::*;

		terminal::TERMINAL.reset();
		FRAMEBUFFER.draw_rect(0, 0, (FRAMEBUFFER.width - 1) as usize, (FRAMEBUFFER.height - 1) as usize, 0xFF0000);

		kprintln!("{}", info);

		hang()
	}
}

unsafe fn hang() -> ! {
	core::arch::asm!("cli");
	loop { core::arch::asm!("hlt"); }
}