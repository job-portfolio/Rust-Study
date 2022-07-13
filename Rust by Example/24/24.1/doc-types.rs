#[doc(inline)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}



// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;



// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;
