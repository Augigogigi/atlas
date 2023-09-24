use lazy_static::lazy_static;
use x86_64::{
	instructions::{
		segmentation::{Segment, CS, SS},
		tables,
	},
	structures::{
		gdt::{GlobalDescriptorTable, SegmentSelector, Descriptor},
		tss::TaskStateSegment,
	},
	VirtAddr,
};

pub const IST_DOUBLE_FAULT_INDEX: usize = 0;

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
	
		tss.interrupt_stack_table[IST_DOUBLE_FAULT_INDEX] = unsafe {
			VirtAddr::new(crate::boot::KERNEL_DATA.stack_addr as u64) + crate::boot::STACK_SIZE
		};
	
		tss
	};
}

lazy_static! {
	static ref GDT: (GlobalDescriptorTable, (SegmentSelector, SegmentSelector)) = {
		let mut gdt = GlobalDescriptorTable::new();
		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
		(gdt, (code_selector, tss_selector))
	};
}

pub fn init() {
	GDT.0.load();
	unsafe {
		SS::set_reg(SegmentSelector(0));
		CS::set_reg(GDT.1.0);
		tables::load_tss(GDT.1.1);
	}
}