#![cfg_attr(not(test), no_std)]

use log::{LevelFilter, Log, Level};

use core::fmt::{self, Write};
use core::result::Result::Ok;

pub trait LogFuncs {
    fn console_write_str(&self, s: &str);
}

struct Logger;

macro_rules! with_color {
    ($color_code:expr, $($arg:tt)*) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[m", $color_code as u8, format_args!($($arg)*))
    }};
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // self.inner.console_write_str(s);
        for c in s.chars() {
            // 0x3F20_1000 raspi3b
            // unsafe { core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8) }
            match c as u8 {
                b'\n' => unsafe {
                    core::ptr::write_volatile(0x0900_0000 as *mut u8, b'\r');
                    core::ptr::write_volatile(0x0900_0000 as *mut u8, b'\n');
                },
                c => unsafe { core::ptr::write_volatile(0x0900_0000 as *mut u8, c as u8) },
            }
        }
        Ok(())
    }
}

enum ColorCode {
    Red = 31,
    Yellow = 33,
    Green = 32,
    Cyan = 36,
    BrightBlack = 90,
}

impl Log for Logger {
    #[inline]
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = record.level();
        let color = match level {
            Level::Error => ColorCode::Red,
            Level::Warn => ColorCode::Yellow,
            Level::Info => ColorCode::Green,
            Level::Debug => ColorCode::Cyan,
            Level::Trace => ColorCode::BrightBlack,
        };
        console_write(with_color!(color, "{}\n", record.args()));
    }

    fn flush(&self) {}
}

pub fn console_write(args: fmt::Arguments) {
    Logger.write_fmt(args).unwrap();
}

pub fn init() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(LevelFilter::Trace);
}
