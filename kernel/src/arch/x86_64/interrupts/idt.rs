use lazy_static::lazy_static;
use x86_64::structures::idt::{
	InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
};

pub const PIC_1_OFFSET: usize = 32;
pub const PIC_2_OFFSET: usize = PIC_1_OFFSET + 8;

const PIC_OFFSET_TIMER: usize = PIC_1_OFFSET;
const PIC_OFFSET_KEYBOARD: usize = PIC_OFFSET_TIMER + 1;

pub static PICS: spin::mutex::Mutex<pic8259::ChainedPics> = unsafe {
	spin::mutex::Mutex::new(pic8259::ChainedPics::new(PIC_1_OFFSET as u8, PIC_2_OFFSET as u8))
};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(super::gdt::IST_DOUBLE_FAULT_INDEX as u16) };

        idt[PIC_OFFSET_TIMER].set_handler_fn(timer_interrupt_handler);
        idt[PIC_OFFSET_KEYBOARD].set_handler_fn(keyboard_interrupt_handler);

        idt.divide_error.set_handler_fn(fault_handler::<"DIVIDE ERROR">);
        idt.debug.set_handler_fn(fault_handler::<"DEBUG">);
        idt.non_maskable_interrupt.set_handler_fn(fault_handler::<"NON-MASKABLE INTERRUPT">);
        idt.breakpoint.set_handler_fn(fault_handler::<"BREAKPOINT">);
        idt.overflow.set_handler_fn(fault_handler::<"OVERFLOW">);
        idt.bound_range_exceeded.set_handler_fn(fault_handler::<"BOUND RANGE EXCEEDED">);
        idt.invalid_opcode.set_handler_fn(fault_handler::<"INVALID OPCODE">);
        idt.device_not_available.set_handler_fn(fault_handler::<"DEVICE NOT AVAILABLE">);
        idt.invalid_tss.set_handler_fn(fault_handler_with_code::<"INVALID TSS">);
        idt.segment_not_present.set_handler_fn(fault_handler_with_code::<"SEGMENT NOT PRESENT">);
        idt.stack_segment_fault.set_handler_fn(fault_handler_with_code::<"STACK SEGMENT FAULT">);
        idt.general_protection_fault.set_handler_fn(fault_handler_with_code::<"GENERAL PROTECTION FAULT">);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(fault_handler::<"x87 FLOATING POINT">);
        idt.alignment_check.set_handler_fn(fault_handler_with_code::<"ALIGNMENT CHECK">);
        idt.machine_check.set_handler_fn(diverging_fault_handler::<"MACHINE CHECK">);
        idt.simd_floating_point.set_handler_fn(fault_handler::<"SIMD FLOATING POINT">);
        idt.virtualization.set_handler_fn(fault_handler::<"VIRTUALIZATION">);
        idt.security_exception.set_handler_fn(fault_handler_with_code::<"SECURITY EXCEPTION">);

        idt
    };
}

pub extern "x86-interrupt" fn fault_handler<const S: &'static str>(
	stack_frame: InterruptStackFrame,
) {
	panic!("EXCEPTION: {}!\nStack Frame: {:#?}", S, stack_frame);
}

pub extern "x86-interrupt" fn fault_handler_with_code<const S: &'static str>(
	stack_frame: InterruptStackFrame,
	error_code: u64,
) {
	panic!(
        "EXCEPTION: {}!\nError Code: {},\nStack Frame: {:#?}",
        S,
        error_code,
        stack_frame
    );
}

pub extern "x86-interrupt" fn diverging_fault_handler<const S: &'static str>(
	stack_frame: InterruptStackFrame,
) -> ! {
	panic!("EXCEPTION: {}!\nStack Frame: {:#?}", S, stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
	stack_frame: InterruptStackFrame,
	_error_code: u64,
) -> ! {
	panic!("EXCEPTION: DOUBLE FAULT!\nStack Frame: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(
	stack_frame: InterruptStackFrame,
	error_code: PageFaultErrorCode,
) {
	panic!(
        "EXCEPTION: PAGE FAULT!\nAccessed Address: {:?}\nError Code: {:?}\nStack Frame: {:#?}",
        x86_64::registers::control::Cr2::read(),
        error_code,
        stack_frame
    );
}

// Interrupt Handlers
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(PIC_OFFSET_TIMER as u8);
	}
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
	let mut port = x86_64::instructions::port::Port::new(0x60);
	let scancode: u8 = unsafe { port.read() };

	crate::print!("{} ", scancode);

	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(PIC_OFFSET_KEYBOARD as u8);
	}
}

pub fn init() {
	IDT.load();
}