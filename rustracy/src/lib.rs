pub use tracy_macros::{zone_scoped, zone_scoped_prefix};
pub use tracy_sys::*;

#[macro_export]
macro_rules! message_string_literal {
    ($name:tt) => {
        rustracy::emit_message_string_literal(concat!($name, "\0"))
    };
}
