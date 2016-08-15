//! Architecture support for x86_64

#![feature(asm)]
#![feature(concat_idents)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![no_std]

#[macro_use]
extern crate bitflags;

/// Print to console
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::serial::SerialConsole::new(), $($arg)*);
    });
}

/// Print with new line to console
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Create an interrupt function that can safely run rust code
#[macro_export]
macro_rules! interrupt {
    ($name:ident, $func:block) => {
        #[naked]
        pub unsafe extern fn $name () {
            unsafe fn inner() {
                $func
            }

            // Push scratch registers
            asm!("push rax
                push rcx
                push rdx
                push rdi
                push rsi
                push r8
                push r9
                push r10
                push r11"
                : : : : "intel", "volatile");

            // Call inner rust function
            inner();

            // Pop scratch registers and return
            asm!("pop r11
                pop r10
                pop r9
                pop r8
                pop rsi
                pop rdi
                pop rdx
                pop rcx
                pop rax
                iretq"
                : : : : "intel", "volatile");
        }
    };
}

/// Global descriptor table
pub mod gdt;

/// Interrupt descriptor table
pub mod idt;

/// IO Handling
pub mod io;

/// Interrupt instructions
pub mod interrupt;

/// Initialization and main function
pub mod main;

/// Memcpy, memmove, etc.
pub mod mem;

/// Serial driver and print! support
pub mod serial;

/// Task state segment
pub mod tss;

pub mod physical;