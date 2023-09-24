pub mod init;

use crate::api::syscall::id;

#[doc(hidden)]
#[inline(always)]
pub fn syscall(call_id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
	match call_id {
		id::INIT => init::init(),
		_ => unimplemented!()
	}
}
