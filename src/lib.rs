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

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(core_intrinsics)]
#![feature(panic_handler)]
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
