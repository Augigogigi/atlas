// Kernel Options
pub const STACK_SIZE: usize = 16_384;

// Kernel Boot Data
pub struct KernelData {
	pub kernel_p_addr: usize,
	pub kernel_v_addr: usize,
	pub stack_addr: usize,
	pub framebuffer: core::option::Option<&'static limine::NonNullPtr<limine::Framebuffer>>,
}

pub static mut KERNEL_DATA: KernelData = KernelData {
	kernel_p_addr: 0,
	kernel_v_addr: 0,
	stack_addr: 0,
	framebuffer: None,
};

// Limine Requests
pub static KERNEL_ADDRESS_REQUEST: limine::KernelAddressRequest = limine::KernelAddressRequest::new(0);
pub static STACK_SIZE_REQUEST: limine::StackSizeRequest = limine::StackSizeRequest::new(0).stack_size(STACK_SIZE as u64);
pub static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);