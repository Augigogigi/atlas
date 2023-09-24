pub mod id;

pub fn syscall(call_id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
	#[cfg(target_arch = "x86_64")]
	return crate::arch::x86_64::syscall::syscall(call_id, arg1, arg2, arg3, arg4);

	#[cfg(target_arch = "aarch64")]
	return crate::arch::aarch64::syscall::syscall(call_id, arg1, arg2, arg3, arg4);
	
	#[cfg(target_arch = "riscv64")]
	return crate::arch::riscv64::syscall::syscall(call_id, arg1, arg2, arg3, arg4);
}

pub fn init() -> usize {
	syscall(id::INIT, 0, 0, 0, 0)
}