pub fn init() -> usize {
	/* Get Framebuffer */
	if let Some(framebuffer_response) = crate::boot::FRAMEBUFFER_REQUEST.get_response().get() {
		if let Some(framebuffer) = framebuffer_response.framebuffers().get(0) {
			unsafe {
				crate::boot::KERNEL_DATA.framebuffer = Some(framebuffer);
			}
		} else {
			panic!("Limine did not pass a valid framebuffer!");
		}
	} else {
		panic!("Limine did not respond to FRAMEBUFFER_REQUEST!");
	}
	crate::success!("Booted with valid framebuffer!");

	/* Get Kernel Address */
	if let Some(kernel_address_response) = crate::boot::KERNEL_ADDRESS_REQUEST.get_response().get() {
		unsafe {
			crate::boot::KERNEL_DATA.kernel_p_addr = kernel_address_response.physical_base as usize;
			crate::boot::KERNEL_DATA.kernel_v_addr = kernel_address_response.virtual_base as usize;
		}
	} else {
		panic!("Limine did not respond to KERNEL_ADDRESS_REQUEST!");
	}
	crate::info!("KernelPAddr: {:#018X}", unsafe { crate::boot::KERNEL_DATA.kernel_p_addr });
	crate::info!("KernelVAddr: {:#018X}", unsafe { crate::boot::KERNEL_DATA.kernel_v_addr });

	/* Get Stack Address */
	unsafe {
		let stack_pointer: usize;
		core::arch::asm! {
			"mov {stack_pointer}, rsp",
			stack_pointer = out(reg) stack_pointer
		};
		crate::boot::KERNEL_DATA.stack_addr = stack_pointer;
	}

	/* Check Stack Size */
	if let None = crate::boot::STACK_SIZE_REQUEST.get_response().get() {
		panic!("Limine did not respond to STACK_SIZE_REQUEST!");
	}
	crate::info!("StackAddr: {:#018X}", unsafe { crate::boot::KERNEL_DATA.stack_addr });
	crate::info!("StackSize: {} Bytes", crate::boot::STACK_SIZE);

	/* Load GDT */
	crate::arch::x86_64::interrupts::gdt::init();
	crate::success!("GDT Initialized!");

	/* Load IDT */
	crate::arch::x86_64::interrupts::idt::init();
	crate::success!("IDT Initialized!");

	return 0;
}