#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(strict_provenance)]
#![feature(adt_const_params)]
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
			error!("{}", info);
		}

		hang();
	}
}

unsafe fn hang() -> ! {
	core::arch::asm!("cli");
	loop { core::arch::asm!("hlt"); }
}