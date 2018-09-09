//! Set the panicking behavior to halt
//!
//! This crate contains an implementation of `panic_fmt` that simply halt in an infinite loop.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_halt;
//!
//! fn main() {
//!     panic!("argument is ignored");
//! }
//! ```
//!
//! # Breakable symbols
//!
//! With the panic handler being `#[inline(never)]` the symbol `rust_begin_unwind` will be
//! available to place a breakpoint on to halt when a panic is happening.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
