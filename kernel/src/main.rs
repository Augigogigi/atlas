#![no_std]
#![no_main]

#![feature(strict_provenance)]
#![feature(const_mut_refs)]

pub mod api;
pub mod arch;
pub mod boot;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
	api::syscall::init();

	hang();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	unsafe {
		if let Some(_) = boot::KERNEL_DATA.framebuffer {
			use api::display::*;
	
			terminal::TERMINAL.reset();
			framebuffer::FRAMEBUFFER.draw_rect(0, 0, (framebuffer::FRAMEBUFFER.width - 1) as usize, (framebuffer::FRAMEBUFFER.height - 1) as usize, 0xFF0000);

			kprintln!("{}", info);
		}

		hang()
	}
}

unsafe fn hang() -> ! {
	core::arch::asm!("cli");
	loop { core::arch::asm!("hlt"); }
}