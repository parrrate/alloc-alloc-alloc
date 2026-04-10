//! `alloc::alloc::alloc` is quite repetitive to write, so we make a convenient `alloc_alloc_alloc`
//! macro to simplify the call:
//!
//! ```rust
//! # use alloc::alloc::{dealloc, handle_alloc_error, Layout};
//! # extern crate alloc;
//! #
//! unsafe {
//!     let layout = Layout::new::<u16>();
//!     let ptr = alloc_alloc_alloc::alloc_alloc_alloc!(layout);
//!     if ptr.is_null() {
//!         handle_alloc_error(layout);
//!     }
//!
//!     *(ptr as *mut u16) = 42;
//!     assert_eq!(*(ptr as *mut u16), 42);
//! #
//! #    dealloc(ptr, layout);
//! }
//! ```

#![no_std]

extern crate alloc;

#[macro_export]
macro_rules! alloc_alloc_alloc {
    ($layout:expr) => {
        alloc::alloc::alloc($layout)
    };
}
