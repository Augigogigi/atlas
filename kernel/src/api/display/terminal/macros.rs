
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
		$crate::api::display::terminal::_print(format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! println {
    () => {
		$crate::print!("\n")
	};
    ($($arg:tt)*) => {
		$crate::print!("{}\n", format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
		unsafe {
			$crate::print!("[ ");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::INFO_COLOR);
			$crate::print!("INFO");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::TEXT_COLOR);
			$crate::print!(" ] ");
		}
		$crate::print!("{}\n", format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
		unsafe {
			$crate::print!("[ ");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::SUCCESS_COLOR);
			$crate::print!("SUCCESS");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::TEXT_COLOR);
			$crate::print!(" ] ");
		}
		$crate::print!("{}\n", format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
		unsafe {
			$crate::print!("[ ");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::WARNING_COLOR);
			$crate::print!("WARNING");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::TEXT_COLOR);
			$crate::print!(" ] ");
		}
		$crate::print!("{}\n", format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
		unsafe {
			$crate::print!("[ ");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::ERROR_COLOR);
			$crate::print!("ERROR");
			$crate::api::display::terminal::TERMINAL.color($crate::api::display::TEXT_COLOR);
			$crate::print!(" ] ");
		}
		$crate::print!("{}\n", format_args!($($arg)*))
	};
}